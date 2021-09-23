pub mod response;

use crate::config::MailConfig;
use rocket::State;
use rocket::response::content::Xml;
use serde::Serialize;

#[get("/.well-known/autoconfig/mail/config-v1.1.xml?<emailaddress>")]
pub fn autoconfig_wellknown(config: &State<MailConfig>, emailaddress: String) -> Xml<String> {
    autoconfig(config, emailaddress)
}

#[get("/mail/config-v1.1.xml?<emailaddress>")]
pub fn autoconfig(config: &State<MailConfig>, emailaddress: String) -> Xml<String> {
    let domain = emailaddress.split("@").last().unwrap();
    let domain = config.domains.get(domain).unwrap();

    let mut incoming_servers = vec![];
    let mut outgoing_servers = vec![];

    for server in &domain.servers {
        let socket_ty = match server.encrypt.as_str() {
            "ssl" => "SSL".to_string(),
            "starttls" => "STARTTLS".to_string(),
            _ => "plain".to_string(),
        };

        let auth = match server.auth.as_str() {
            _ => "password-cleartext".to_string(),
        };

        let item = response::Server {
            ty: server.protocol.clone(),
            hostname: response::Hostname(server.hostname.clone()),
            port: response::Port(server.port),
            socket_type: response::SocketType(socket_ty),
            auth: response::Authentication(auth),
            username: response::Username(emailaddress.clone()),
        };

        if server.protocol == "smtp" {
            outgoing_servers.push(item);
        } else {
            incoming_servers.push(item);
        }
    }

    let response = response::ClientConfig {
        version: "1.1".to_string(),
        providers: vec![
            response::Provider {
                id: "synkhronix.com".to_string(),
                domain: response::Domain(domain.domain.clone()),
                display_name: response::DisplayName(domain.name.clone()),
                display_short_name: response::ShortName(domain.short_name.clone()),
                incoming_servers,
                outgoing_servers,
            },
        ],
    };

    let mut data = Vec::new();
    let writer = quick_xml::Writer::new(&mut data);
    let mut serializer = quick_xml::se::Serializer::with_root(writer, Some("clientConfig"));
    let _ = response.serialize(&mut serializer);
    let response = String::from_utf8(data).unwrap();

    Xml(format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>{}", response))
}

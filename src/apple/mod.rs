use crate::config::{MailConfig, SSL};
use rocket::State;
use rocket::response::content::Xml;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct PayloadContent {
    email_account_type: String,
    email_address: String,

    incoming_mail_server_authentication: String,
    incoming_mail_server_host_name: String,
    incoming_mail_server_port_number: u16,
    incoming_mail_server_use_ssl: bool,

    outgoing_mail_server_authentication: String,
    outgoing_mail_server_host_name: String,
    outgoing_mail_server_port_number: u16,
    outgoing_mail_server_use_ssl: bool,

    payload_description: String,
    payload_display_name: String,
    payload_identifier: String,
    payload_organization: String,
    payload_type: String,
    #[serde(rename = "PayloadUUID")]
    payload_uuid: Uuid,
    payload_version: u32,

    prevent_app_sheet: bool,
    prevent_move: bool,
    #[serde(rename = "SMIMEEnabled")]
    smime_enabled: bool,
    #[serde(rename = "allowMailDrop")]
    allow_mail_drop: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct MobileConfig {
    payload_content: Vec<PayloadContent>,
    payload_description: String,
    payload_display_name: String,
    payload_identifier: String,
    payload_organization: String,
    payload_removal_disallowed: bool,
    payload_type: String,
    #[serde(rename = "PayloadUUID")]
    payload_uuid: Uuid,
    payload_version: u32,
}

struct Server {
    auth: String,
    hostname: String,
    port: u16,
    use_ssl: bool,
}

use openssl::pkcs7::{Pkcs7, Pkcs7Flags};
use openssl::pkey::PKey;
use openssl::stack::Stack;
use openssl::x509::X509;

fn sign(ssl: &SSL, input: &[u8]) -> Result<Vec<u8>, failure::Error> {
    let data = std::fs::read(&ssl.key)?;
    let pkey = PKey::private_key_from_pem(&data)?;

    let data = std::fs::read(&ssl.chain)?;
    let cert = X509::from_pem(&data)?;
    let mut certs = Stack::new()?;
    certs.push(cert)?;

    let data = std::fs::read(&ssl.cert)?;
    let cert = X509::from_pem(&data)?;

    let flags = Pkcs7Flags::STREAM;
    let pkcs7 = Pkcs7::sign(&cert, &pkey, &certs.as_ref(), input, flags)?;
    let data = pkcs7.to_der()?;

    Ok(data)
}

#[get("/email.mobileconfig?<email>")]
pub fn mobileconfig(config: &State<MailConfig>, email: String) -> Xml<Vec<u8>> {
    let domain = email.split("@").last().unwrap().to_string();
    let identifier = domain.split('.').into_iter().rev().collect::<Vec<&str>>().join(".");
    let domain_config = config.domains.get(&domain).unwrap();

    let mut incoming_server = None;
    let mut outgoing_server = None;

    for server in &domain_config.servers {
        let use_ssl = match server.encrypt.as_str() {
            "ssl" => true,
            "starttls" => true,
            _ => false,
        };

        let auth = match server.auth.as_str() {
            "plain" => "EmailAuthPassword",
            "cram-md5" => "EmailAuthCRAMMD5",
            _ => "EmailAuthNone",
        };

        let item = Server {
            auth: auth.to_string(),
            hostname: server.hostname.clone(),
            port: server.port,
            use_ssl,
        };

        if server.protocol == "smtp" {
            outgoing_server.get_or_insert(item);
        } else {
            incoming_server.get_or_insert(item);
        }
    }

    let incoming_server = incoming_server.unwrap();
    let outgoing_server = outgoing_server.unwrap();

    let response = MobileConfig {
        payload_content: vec![PayloadContent {
            email_account_type: "EmailTypeIMAP".to_string(),
            email_address: email.clone(),

            incoming_mail_server_authentication: incoming_server.auth,
            incoming_mail_server_host_name: incoming_server.hostname,
            incoming_mail_server_port_number: incoming_server.port,
            incoming_mail_server_use_ssl: incoming_server.use_ssl,

            outgoing_mail_server_authentication: outgoing_server.auth,
            outgoing_mail_server_host_name: outgoing_server.hostname,
            outgoing_mail_server_port_number: outgoing_server.port,
            outgoing_mail_server_use_ssl: outgoing_server.use_ssl,

            payload_description: "Configures your e-mail account.".to_string(),
            payload_display_name: "IMAP Account".to_string(),
            payload_identifier: format!("{}.mobileconfig", identifier),
            payload_organization: domain.clone(),
            payload_type: "com.apple.mail.managed".to_string(),
            payload_uuid: Uuid::new_v4(),
            payload_version: 1,

            prevent_app_sheet: false,
            prevent_move: false,
            smime_enabled: false,
            allow_mail_drop: true,
        }],

        payload_description: format!("Install this profile to automatically configure the e-mail account for {}", email),
        payload_display_name: "Mail Account".to_string(),
        payload_identifier: format!("{}.mobileconfig", identifier),
        payload_organization: domain.clone(),
        payload_removal_disallowed: false,
        payload_type: "Configuration".to_string(),
        payload_uuid: Uuid::new_v4(),
        payload_version: 1,
    };

    let mut data = vec![];
    let _ = plist::to_writer_xml(&mut data, &response);
    let response = String::from_utf8(data).unwrap();
    let response = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>{}", response);
    let mut response = response.into_bytes();

    if let Some(ssl) = &config.ssl {
        response = sign(ssl, &response).unwrap();
    }

    Xml(response)
}

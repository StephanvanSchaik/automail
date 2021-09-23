mod request;
mod response;

use crate::config::MailConfig;
use rocket::{Data, Request, State};
use rocket::data::{self, FromData, ToByteUnit};
use rocket::http::Status;
use rocket::response::content::Xml;
use std::collections::HashMap;

pub struct Body {
    pub body: String,
}

#[derive(Debug)]
pub enum BodyError {
    Io(std::io::Error),
    TooLarge,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Body {
    type Error = BodyError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;

        // Use a configured limit with name 'body' or fallback to default.
        let limit = req.limits().get("body").unwrap_or(4096_i32.bytes());

        // Read the data into a string.
        let body = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, BodyError::TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, BodyError::Io(e))),
        };

        Success(Body { body: body })
    }
}

#[post("/autodiscover/autodiscover.xml", data = "<data>")]
pub fn autodiscover(config: &State<MailConfig>, data: Body) -> Xml<String> {
    let error_response = response::Autodiscover {
        xmlns: None,
        response: response::Response {
            xmlns: None,
            body: response::ResponseBody::Error {
                error_code: response::ErrorCode(600),
                message: response::Message("Invalid Request".to_string()),
            },
        },
    };

    let request: request::Autodiscover = match quick_xml::de::from_str(&data.body) {
        Ok(request) => request,
        _ => return Xml(format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>{}", quick_xml::se::to_string(&error_response).unwrap())),
    };

    let domain = request.request.email.split("@").last().unwrap();
    let domain = config.domains.get(domain).unwrap();

    let mut protocols = HashMap::new();

    for server in &domain.servers {
        let protocol = server.protocol.to_uppercase();

        if protocols.contains_key(&protocol) {
            continue;
        }

        let use_ssl = match server.encrypt.as_str() {
            "ssl" => "on".to_string(),
            "starttls" => "on".to_string(),
            _ => "off".to_string(),
        };

        let encryption = match server.encrypt.as_str() {
            "ssl" => Some(response::Encryption("ssl".to_string())),
            "starttls" => Some(response::Encryption("tls".to_string())),
            _ => None,
        };

        let item = response::Protocol {
            ty: response::ProtocolType(protocol.clone()),
            server: response::Server(server.hostname.clone()),
            port: response::Port(server.port),
            login_name: response::LoginName(request.request.email.clone()),
            domain_required: response::DomainRequired("on".to_string()),
            auth_required: response::AuthRequired("on".to_string()),
            use_ssl: Some(response::UseSSL(use_ssl)),
            use_spa: response::UseSPA("off".to_string()),
            encryption,
        };

        protocols.insert(protocol, item);
    }

    let protocols = protocols
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<response::Protocol>>();

    let response = response::Autodiscover {
        xmlns: Some("http://schemas.microsoft.com/exchange/autodiscover/responseschema/2006".to_string()),
        response: response::Response {
            xmlns: Some("http://schemas.microsoft.com/exchange/autodiscover/outlook/responseschema/2006a".to_string()),
            body: response::ResponseBody::Account {
                account_type: response::AccountType("email".to_string()),
                action: response::Action("settings".to_string()),
                protocols,
            },
        },
    };

    Xml(format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>{}", quick_xml::se::to_string(&response).unwrap()))
}

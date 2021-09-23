use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorCode(pub u32);

#[derive(Debug, Serialize)]
pub struct Message(pub String);

#[derive(Debug, Serialize)]
pub struct AccountType(pub String);

#[derive(Debug, Serialize)]
pub struct Action(pub String);

#[derive(Debug, Serialize)]
pub struct ProtocolType(pub String);

#[derive(Debug, Serialize)]
pub struct Server(pub String);

#[derive(Debug, Serialize)]
pub struct Port(pub u16);

#[derive(Debug, Serialize)]
pub struct LoginName(pub String);

#[derive(Debug, Serialize)]
pub struct DomainRequired(pub String);

#[derive(Debug, Serialize)]
pub struct AuthRequired(pub String);

#[derive(Debug, Serialize)]
pub struct UseSSL(pub String);

#[derive(Debug, Serialize)]
pub struct UseSPA(pub String);

#[derive(Debug, Serialize)]
pub struct Encryption(pub String);

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Protocol {
    #[serde(rename = "Type")]
    pub ty: ProtocolType,
    pub server: Server,
    pub port: Port,
    pub login_name: LoginName,
    pub domain_required: DomainRequired,
    pub auth_required: AuthRequired,
    #[serde(rename = "SSL")]
    pub use_ssl: Option<UseSSL>,
    #[serde(rename = "SPA")]
    pub use_spa: UseSPA,
    pub encryption: Option<Encryption>,
}

#[derive(Debug, Serialize)]
pub enum ResponseBody {
    #[serde(rename_all = "PascalCase")]
    Error {
        error_code: ErrorCode,
        message: Message,
    },
    #[serde(rename_all = "PascalCase")]
    Account {
        account_type: AccountType,
        action: Action,
        #[serde(rename = "Protocol", default)]
        protocols: Vec<Protocol>,
    },
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub xmlns: Option<String>,
    pub body: ResponseBody,
}

#[derive(Debug, Serialize)]
pub struct Autodiscover {
    pub xmlns: Option<String>,
    #[serde(rename = "Response")]
    pub response: Response,
}

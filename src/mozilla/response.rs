use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Domain(pub String);

#[derive(Debug, Serialize)]
pub struct DisplayName(pub String);

#[derive(Debug, Serialize)]
pub struct ShortName(pub String);

#[derive(Debug, Serialize)]
pub struct Hostname(pub String);

#[derive(Debug, Serialize)]
pub struct Port(pub u16);

#[derive(Debug, Serialize)]
pub struct SocketType(pub String);

#[derive(Debug, Serialize)]
pub struct Authentication(pub String);

#[derive(Debug, Serialize)]
pub struct Username(pub String);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "type")]
    pub ty: String,
    pub hostname: Hostname,
    pub port: Port,
    pub socket_type: SocketType,
    pub auth: Authentication,
    pub username: Username,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub id: String,
    pub domain: Domain,
    pub display_name: DisplayName,
    pub display_short_name: ShortName,
    #[serde(rename = "incomingServer")]
    pub incoming_servers: Vec<Server>,
    #[serde(rename = "outgoingServer")]
    pub outgoing_servers: Vec<Server>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfig {
    pub version: String,
    #[serde(rename = "emailProvider")]
    pub providers: Vec<Provider>,
}

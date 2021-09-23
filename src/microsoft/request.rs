use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    #[serde(rename = "EMailAddress")]
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct Autodiscover {
    #[serde(rename = "Request")]
    pub request: Request,
}

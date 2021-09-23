use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub protocol: String,
    pub hostname: String,
    pub port: u16,
    pub auth: String,
    pub encrypt: String,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub domain: String,
    pub name: String,
    pub short_name: String,
    #[serde(rename = "server", default)]
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct SSL {
    pub chain: String,
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "domain", default)]
    pub domains: Vec<Domain>,
    pub ssl: Option<SSL>,
}

#[derive(Debug)]
pub struct MailConfig {
    pub domains: HashMap<String, Domain>,
    pub ssl: Option<SSL>,
}

impl MailConfig {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, failure::Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        let domains = config.domains
            .into_iter()
            .map(|domain| (domain.domain.clone(), domain))
            .collect::<HashMap<String, Domain>>();

        Ok(MailConfig {
            domains,
            ssl: config.ssl,
        })
    }
}

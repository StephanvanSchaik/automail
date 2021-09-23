#[macro_use]
extern crate rocket;

pub mod apple;
pub mod config;
pub mod microsoft;
pub mod mozilla;

#[launch]
fn rocket() -> _ {
    let config = config::MailConfig::read("/etc/automail/config.toml")
        .or(config::MailConfig::read("config.toml"))
        .expect("error: unable to load the configuration file");

    rocket::build()
        .manage(config)
        .mount("/", routes![apple::mobileconfig, mozilla::autoconfig, mozilla::autoconfig_wellknown, microsoft::autodiscover])
}

use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
// Represent our app settings as a Rust type that implements Deserialize
pub struct Settings {
    pub database: DBSettings,
    pub app_port: u16,
}

#[derive(Deserialize)]
pub struct DBSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Initialise configuration reader using ConfigBuilder
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DBSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}

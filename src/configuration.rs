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
    // Initialise configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a 'configuration' file
    // Looks for top-level file with an extensions tht 'config' can parse
    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}

impl DBSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}

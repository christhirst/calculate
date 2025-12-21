use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub url: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::with_prefix("APP").separator("__"));

        builder.build()?.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_load() {
        let settings = Settings::new();
        assert!(
            settings.is_ok(),
            "Settings failed to load: {:?}",
            settings.err()
        );
        let settings = settings.unwrap();

        // Check default values from config/default.toml
        assert_eq!(settings.url, "http://[::]:50051");
    }
}

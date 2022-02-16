use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chainsaw {
    pub grpc: GRPC,
}

#[derive(Deserialize)]
pub struct GRPC {
    pub address: String,
    pub port: u16,
}

/// Reads application configuration from either a `chainsaw.toml` file, or from
/// environment variables.
pub fn get_configuration() -> Result<Chainsaw, config::ConfigError> {
    let mut settings = Config::default();

    settings.merge(config::File::with_name("chainsaw"))?;
    settings.merge(config::Environment::default())?;

    settings.try_into()
}

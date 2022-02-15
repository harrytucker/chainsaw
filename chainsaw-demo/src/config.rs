use std::net::{IpAddr, SocketAddr};

use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chainsaw {
    pub grpc: GRPC,

    pub log: Log,
}

#[derive(Deserialize)]
pub struct GRPC {
    #[serde(default = "default_grpc_address")]
    pub address: IpAddr,
    pub port: u16,
}

impl GRPC {
    pub fn serve_addr(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }
}

#[derive(Deserialize)]
pub struct Log {
    #[serde(default = "default_log_level")]
    pub level: String,
}

/// Reads application configuration from either a `chainsaw.toml` file, or from
/// environment variables.
pub fn get_configuration() -> Result<Chainsaw, config::ConfigError> {
    let mut settings = Config::default();

    settings.merge(config::File::with_name("chainsaw"))?;
    settings.merge(config::Environment::default())?;

    settings.try_into()
}

fn default_grpc_address() -> IpAddr {
    "0.0.0.0".parse().expect("failed to parse default grpc address")
}

fn default_log_level() -> String {
    "info".to_string()
}

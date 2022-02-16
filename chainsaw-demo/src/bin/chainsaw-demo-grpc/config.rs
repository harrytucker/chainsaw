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
    #[serde(default)]
    pub level: LogLevel,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
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
    "0.0.0.0"
        .parse()
        .expect("failed to parse default grpc address")
}

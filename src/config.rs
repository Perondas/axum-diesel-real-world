use envconfig::Envconfig;
use tokio::sync::OnceCell;

#[derive(Debug, Envconfig)]
pub struct Config {
    #[envconfig(nested = true)]
    pub server: ServerConfig,
    #[envconfig(nested = true)]
    pub db: DatabaseConfig,
}

#[derive(Debug, Envconfig)]
pub struct ServerConfig {
    #[envconfig(from = "HOST", default = "0.0.0.0")]
    pub host: String,
    #[envconfig(from = "PORT", default = "3000")]
    pub port: u16,
}

#[derive(Debug, Envconfig)]
pub struct DatabaseConfig {
    #[envconfig(from = "DATABASE_URL")]
    pub url: String,
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn config() -> &'static Config {
    CONFIG
        .get_or_init(|| async { Config::init_from_env().unwrap() })
        .await
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub mongo: MongoConfig,
    pub rabbitmq: RabbitMQConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_app_username")]
    pub username: String,
    #[serde(default = "default_app_password")]
    pub password: String,
    #[serde(default = "default_app_jwt_secret")]
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct MongoConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMQConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

const DEFAULT_CONFIG_FILE: &str = "/path/to/config/file";

fn default_base_url() -> String {
    "http://localhost:8080".to_string()
}

fn default_app_username() -> String {
    format!("admin@{}", default_base_url())
}

fn default_app_password() -> String {
    "password".to_string()
}

fn default_app_jwt_secret() -> String {
    "secret".to_string()
}
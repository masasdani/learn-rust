
#[derive(Debug, Deserialize)]
pub struct RabbitMQConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
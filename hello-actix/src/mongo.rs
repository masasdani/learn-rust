
use mongodb::{Client, Database, error::Error};

#[derive(Debug, Deserialize)]
pub struct MongoConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl MongoConfig {
    
    pub fn connect(&self) -> Result<Database, Error> {
        let client = Client::with_uri_str(&format!(
            "mongodb://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database            
        ))?;
    
        Ok(client.database(&config.database))
    }

    pub fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 27017,
            username: "root".to_string(),
            password: "root".to_string(),
            database: "test".to_string(),
        }
    }

}

 

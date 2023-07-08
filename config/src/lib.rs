use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: String,
    pub port: u16,
    database_url: String,
    pub max_connection: u32,
}

impl Config {
    pub fn from_env() -> Self {
        
        dotenv().ok();

        let server = env::var("SERVER").expect("SERVER not found in the environment");
        let port: u16 = env::var("PORT")
            .expect("PORT not found in the environment")
            .parse()
            .expect("Failed to parse PORT as u16");
     
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in the environment");
        let max_connection: u32 = env::var("MAX_CONNECTION")
        .expect("MAX_CONNECTION not found in the environment")
        .parse()
        .expect("Failed to parse MAX_CONNECTION as u32");
 
        Config {
            server,
            port,
            database_url,
            max_connection,
        }
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

}


pub async fn create_pool(connect:&str, max_connection:u32) -> Result<sqlx::PgPool, sqlx::Error> {
    // Configure the database connection
    let pool = PgPoolOptions::new()
        .max_connections(max_connection) // Adjust according to your needs
        .connect(connect)
        .await
        .expect("Error building a connection pool");

    Ok(pool)
}

#[cfg(test)]
mod test_super {
    use crate::Config;


    #[test]
    fn test_load_config_from_env() {
        let config = Config::from_env();

        assert_ne!(config.server, "")
    }
}
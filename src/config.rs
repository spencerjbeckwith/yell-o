use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub hostname: String,
    pub port: u32,
    pub elevenlabs_api_key: String,
}

impl Config {
    pub fn get() -> Config {
        dotenv().ok();
        Config {
            hostname: env::var("HOSTNAME")
                .unwrap_or_else(|_| String::from("0.0.0.0")),
            port: env::var("PORT")
                .unwrap_or_else(|_| String::from("5000"))
                .parse()
                .expect("unable to parse PORT environment variable"),
            elevenlabs_api_key: env::var("ELEVENLABS_API_KEY")
                .expect("ELEVENLABS_API_KEY environment variable must be set"),
        }
    }
}
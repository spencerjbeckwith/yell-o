use warp::Filter;
use dotenv::dotenv;
use std::env;
use log::info;
use colog;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    colog::init();
    let config = Config::get();

    let address: SocketAddr = format!("{}:{}", config.hostname, config.port)
        .parse()
        .expect("unable to parse hostname/port");
    info!("Listening at: {address}");

    // Extract me to lib.rs eventually...
    let hello = warp::any().map(|| "Hello!");
    warp::serve(hello).run(address).await;
}

struct Config {
    hostname: String,
    port: u32,
    elevenlabs_api_key: String,
}

impl Config {
    fn get() -> Config {
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
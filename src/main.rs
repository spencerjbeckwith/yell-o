use log::info;
use colog;
use std::net::SocketAddr;
use yell_o::{routes, config::Config};

#[tokio::main]
async fn main() {
    colog::init();
    let config = Config::get();

    let address: SocketAddr = format!("{}:{}", config.hostname, config.port)
        .parse()
        .expect("unable to parse hostname/port");
    info!("Listening at: {address}");
    warp::serve(routes(config)).run(address).await;
}
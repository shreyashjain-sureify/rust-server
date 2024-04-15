#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
    let assets_path = env::var("ASSETS_PATH").unwrap_or(default_path);
    println!("public path: {}", assets_path);
    let server = Server::new("127.0.0.1:8085".to_string());
    server.run(WebsiteHandler::new(assets_path));
}


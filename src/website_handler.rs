use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct WebsiteHandler {
    assets_path: String,
}

impl WebsiteHandler {
    pub fn new(assets_path: String) -> Self {
        Self { assets_path }
    }

    async fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.assets_path, file_path);

        match File::open(path).await {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents).await {
                    println!("Failed to read file: {}", e);
                    None
                } else {
                    Some(contents)
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub server_port: u16,
    pub remote_base: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("SERVER_PORT must be number"),
            remote_base: env::var("REMOTE_BASE")
                .unwrap_or_else(|_| "https://monster-siren.hypergryph.com/api".into()),
        }
    }
}
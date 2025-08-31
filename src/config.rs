use dotenvy::dotenv;
use std::env;
use url::Url;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_port: u16,
    pub remote_base: String,
}

impl Config {
    pub fn from_env() -> Result<Self, crate::error::AppError> {
        dotenv().ok();
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .map_err(|_| crate::error::AppError::Config("SERVER_PORT必须是有效的端口号".into()))?;
        
        let remote_base = env::var("REMOTE_BASE")
            .unwrap_or_else(|_| "https://monster-siren.hypergryph.com/api".into());
        
        // 验证URL格式
        Url::parse(&remote_base)
            .map_err(|_| crate::error::AppError::Config("REMOTE_BASE必须是有效的URL".into()))?;
        
        Ok(Self {
            server_port,
            remote_base,
        })
    }
    
    pub fn server_addr(&self) -> String {
        format!("0.0.0.0:{}", self.server_port)
    }
}

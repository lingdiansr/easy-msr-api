//! # 配置管理模块
//! 
//! 提供应用程序配置的加载和管理功能。
//! 
//! 支持从环境变量加载配置，包括服务器端口和远程API地址。
//! 使用`.env`文件支持本地开发配置。

use dotenvy::dotenv;
use std::env;
use url::Url;

/// 应用程序配置
/// 
/// 包含服务器运行所需的所有配置项。
/// 支持从环境变量加载，并提供合理的默认值。
#[derive(Clone, Debug)]
pub struct Config {
    /// 服务器监听端口
    /// 
    /// 默认值为8080，可通过`SERVER_PORT`环境变量设置
    pub server_port: u16,
    
    /// 远程API基础URL
    /// 
    /// 默认值为MSR官方API地址，可通过`REMOTE_BASE`环境变量设置
    pub remote_base: String,
}

impl Config {
    /// 从环境变量加载配置
    /// 
    /// 加载`.env`文件（如果存在），然后从环境变量读取配置。
    /// 如果环境变量不存在，则使用默认值。
    /// 
    /// # 返回
    /// 
    /// 返回配置实例或配置错误
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use msr_api_rs::config::Config;
    /// 
    /// let config = Config::from_env().unwrap();
    /// println!("服务器端口: {}", config.server_port);
    /// ```
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
    
    /// 获取服务器监听地址
    /// 
    /// 返回格式为`0.0.0.0:端口`的监听地址字符串。
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use msr_api_rs::config::Config;
    /// 
    /// let config = Config::from_env().unwrap();
    /// let addr = config.server_addr();
    /// assert_eq!(addr, "0.0.0.0:8080");
    /// ```
    pub fn server_addr(&self) -> String {
        format!("0.0.0.0:{}", self.server_port)
    }
}

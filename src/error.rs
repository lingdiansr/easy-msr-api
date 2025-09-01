//! # 错误处理模块
//! 
//! 定义了项目中使用的错误类型和错误处理机制。
//! 
//! 提供了统一的错误类型`AppError`，用于处理各种可能的错误情况，
//! 包括网络请求错误、配置错误、资源未找到等。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// 应用程序错误类型
/// 
/// 枚举了所有可能的错误情况，并为每种错误提供了清晰的描述。
/// 实现了`IntoResponse`，可以直接作为Axum的响应返回。
#[derive(Error, Debug)]
pub enum AppError {
    /// 远程API请求错误
    /// 
    /// 包含底层`reqwest::Error`的详细信息
    #[error("远程API请求错误: {0}")]
    Remote(#[from] reqwest::Error),
    
    /// 无效的输入参数
    /// 
    /// 当请求参数不符合要求时返回
    #[error("无效的输入参数: {0}")]
    BadRequest(String),
    
    /// 请求的资源不存在
    /// 
    /// 当请求的资源（歌曲、专辑、新闻等）不存在时返回
    #[error("请求的资源不存在")]
    NotFound,
    
    /// 服务器内部错误
    /// 
    /// 当发生未预期的内部错误时返回
    #[error("服务器内部错误: {0}")]
    Internal(String),
    
    /// 配置错误
    /// 
    /// 当环境变量或配置文件格式不正确时返回
    #[error("配置错误: {0}")]
    Config(String),
}

impl IntoResponse for AppError {
    /// 将错误转换为HTTP响应
    /// 
    /// 根据不同的错误类型返回相应的HTTP状态码和错误信息：
    /// - 请求超时 -> 408 Request Timeout
    /// - 远程服务错误 -> 502 Bad Gateway
    /// - 参数错误 -> 400 Bad Request
    /// - 资源未找到 -> 404 Not Found
    /// - 内部错误 -> 500 Internal Server Error
    /// - 配置错误 -> 500 Internal Server Error
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Remote(ref e) if e.is_timeout() => (StatusCode::REQUEST_TIMEOUT, "请求超时"),
            AppError::Remote(_) => (StatusCode::BAD_GATEWAY, "远程服务暂时不可用"),
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "请求的资源不存在"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "配置错误"),
        };
        
        let body = serde_json::json!({
            "error": message,
            "code": status.as_u16()
        });
        
        (status, axum::Json(body)).into_response()
    }
}

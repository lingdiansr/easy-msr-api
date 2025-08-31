use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("远程API请求错误: {0}")]
    Remote(#[from] reqwest::Error),
    
    #[error("无效的输入参数: {0}")]
    BadRequest(String),
    
    #[error("请求的资源不存在")]
    NotFound,
    
    #[error("服务器内部错误: {0}")]
    Internal(String),
    
    #[error("配置错误: {0}")]
    Config(String),
}

impl IntoResponse for AppError {
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

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 自定义应用错误枚举
#[derive(Debug)]
pub enum AppError {
    // 在这里可以添加更多具体的错误类型，例如数据库错误、未找到等
    InternalServerError,
}

/// 实现 IntoResponse trait，让 AppError 可以被转换为 HTTP 响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

/// 实现 From<reqwest::Error> for AppError
/// 这让我们可以使用 `?` 操作符来自动转换 reqwest 的错误
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        // 在实际应用中，记录详细的错误信息很重要
        tracing::error!("reqwest error: {:?}", err);
        AppError::InternalServerError
    }
}

/// 实现 From<serde_json::Error> for AppError
/// 这让我们可以使用 `?` 操作符来自动转换 serde_json 的错误
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        tracing::error!("serde_json error: {:?}", err);
        AppError::InternalServerError
    }
}

/// 实现 From<std::io::Error> for AppError
/// 这让我们可以使用 `?` 操作符来自动转换 IO 错误
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        tracing::error!("io error: {:?}", err);
        AppError::InternalServerError
    }
}

use crate::client::remote::RemoteApiClient;
use crate::error::AppError;
use crate::web::dto::{CreateUserRequest, User};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "用户 id")
    ),
    responses(
        (status = 200, description = "用户", body = User),
        (status = 404, description = "用户不存在")
    )
)]
pub async fn get_user(
    Path(id): Path<i32>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<User>, AppError> {
    client.get_user(id).await.map(Json)
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "创建成功", body = User),
        (status = 400, description = "参数错误")
    )
)]
pub async fn create_user(
    State(client): State<RemoteApiClient>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    client
        .create_user(&req)
        .await
        .map(|u| (StatusCode::CREATED, Json(u)))
}
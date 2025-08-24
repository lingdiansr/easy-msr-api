use crate::web::dto::{CreateUserRequest, User};
use crate::web::handler::{create_user, get_user};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user, create_user),
    components(schemas(User, CreateUserRequest,SongResp, SongData)),
    // components(schemas(SongResp, SongData)),
    tags(
        (name = "users", description = "用户管理接口")
    )
)]
struct ApiDoc;

pub fn api_doc() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
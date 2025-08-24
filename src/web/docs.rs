use crate::web::dto::*;
use crate::web::handler::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user, create_user, get_song),
    components(schemas(User, CreateUserRequest,SongResp,SongData)),
    tags(
        (name = "users", description = "用户管理接口")
    )
)]
struct ApiDoc;

pub fn api_doc() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

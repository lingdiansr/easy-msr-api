use crate::web::dto::*;
use crate::web::handler::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_song),
    components(schemas( SongResp, SongData)),
    tags(
        (name = "songs", description = "歌曲相关接口"),
    )
)]
struct ApiDoc;

pub fn api_doc() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

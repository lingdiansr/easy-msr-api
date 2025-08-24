use crate::web::dto::*;
use crate::web::handler::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_song, get_all_songs, get_album),
    components(schemas( 
        SongResp, 
        SongData, 
        AllSongsResp, 
        AllSongsItem, 
        AllSongsData, 
        AlbumResp, 
        AlbumData 
    )),
    tags(
        (name = "songs", description = "歌曲相关接口"),
        (name = "albums", description = "专辑相关接口")
    )
)]
struct ApiDoc;

pub fn api_doc() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

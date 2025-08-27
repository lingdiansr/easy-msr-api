use crate::web::dto::*;
use crate::web::handler::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_song, 
        get_all_songs, 
        get_album, 
        get_album_detail,
        get_all_albums,
        search,
        search_album,
    ),
    components(schemas( 
        SongResp, 
        SongData, 

        AllSongsResp, 
        AllSongsData, 
        AllSongsItem, 

        AlbumResp, 
        AlbumData,

        AlbumDetailResp,
        AlbumDetailData,
        AlbumDetailSongItem,

        AllAlbumsItem,
        
        SearchResp,
        SearchData,
        SearchAlbumData,
        SearchAlbumItem,
        NewsData,
        NewsItem,

        SearchAlbumResp,
    )),
    tags(
        (name = "songs", description = "歌曲相关接口"),
        (name = "albums", description = "专辑相关接口"),
        (name = "search", description = "搜索相关接口"),
    )
)]
struct ApiDoc;

pub fn api_doc() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

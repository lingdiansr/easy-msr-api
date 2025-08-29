use crate::client::remote::RemoteApiClient;
use axum::Router;

pub mod docs;
pub mod dto;
pub mod handler;

/// 基础API路由，不包含Swagger UI
pub fn routes(client: RemoteApiClient) -> Router {
    use axum::routing::get;
    use handler::*;

    Router::new()
        .route("/song/{cid}", get(get_song))
        .route("/songs", get(get_all_songs))
        .route("/album/{cid}/data", get(get_album))
        .route("/album/{cid}/detail", get(get_album_detail))
        .route("/albums", get(get_all_albums))
        .route("/news", get(get_all_news))
        .route("/news/{cid}", get(get_news_detail))
        .route("/search", get(search))
        .route("/search/album", get(search_albums))
        .route("/search/news", get(search_news))
        .route("/fontset", get(get_font))
        .with_state(client)
}

/// 包含Swagger UI的API路由
#[cfg(feature = "swagger-ui")]
pub fn routes_with_swagger(client: RemoteApiClient) -> Router {
    use axum::routing::get;
    use handler::*;
    use utoipa_swagger_ui::SwaggerUi;

    Router::new()
        .route("/song/{cid}", get(get_song))
        .route("/songs", get(get_all_songs))
        .route("/album/{cid}/data", get(get_album))
        .route("/album/{cid}/detail", get(get_album_detail))
        .route("/albums", get(get_all_albums))
        .route("/news", get(get_all_news))
        .route("/news/{cid}", get(get_news_detail))
        .route("/search", get(search))
        .route("/search/album", get(search_albums))
        .route("/search/news", get(search_news))
        .route("/fontset", get(get_font))
        .with_state(client)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::api_doc()))
}

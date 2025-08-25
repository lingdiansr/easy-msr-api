use crate::client::remote::RemoteApiClient;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub mod dto;
pub mod handler;

pub fn routes(client: RemoteApiClient) -> Router {
    use axum::routing::get;
    use handler::*;

    Router::new()
        .route("/song/{cid}", get(get_song))
        .route("/songs", get(get_all_songs))
        .route("/album/{cid}/data", get(get_album))
        .route("/album/{cid}/detail", get(get_album_detail))
        .route("/albums", get(get_all_albums))
        .route("/search", get(search))
        .with_state(client)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::api_doc()))
}

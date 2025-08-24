use crate::client::remote::RemoteApiClient;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub mod dto;
pub mod handler;

pub fn routes(client: RemoteApiClient) -> Router {
    use axum::routing::get;
    use handler::{get_song, get_all_songs, get_album};

    Router::new()
        .route("/song/{cid}", get(get_song))
        .route("/songs", get(get_all_songs))
        .route("/album/{cid}/data", get(get_album))
        .with_state(client)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::api_doc()))
}

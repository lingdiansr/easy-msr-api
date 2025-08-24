use crate::client::remote::RemoteApiClient;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub mod dto;
pub mod handler;

pub fn routes(client: RemoteApiClient) -> Router {
    use axum::routing::{get, post};
    use handler::{get_song};

    Router::new()
        .route("/song/{cid}", get(get_song))
        .with_state(client)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::api_doc()))
}

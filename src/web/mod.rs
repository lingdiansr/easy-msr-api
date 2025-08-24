use crate::{client::remote::RemoteApiClient, web::handler::get_song};
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub mod dto;
pub mod handler;

pub fn routes(client: RemoteApiClient) -> Router {
    use axum::routing::{get, post};
    use handler::{create_user, get_user};

    Router::new()
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user))
        .route("/song/{cid}", get(get_song))
        .with_state(client)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::api_doc()))
}

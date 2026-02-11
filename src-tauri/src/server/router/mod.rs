use axum::{routing, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::server::actions::{action_of_upload::upload_handler, actions_of_ip::get_socket_handler};

pub fn api_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/axum-api/socket-ip", routing::get(get_socket_handler))
        .route("/axum-api/upload", routing::post(upload_handler))
        .layer(cors)
}

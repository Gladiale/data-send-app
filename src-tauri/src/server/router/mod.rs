use axum::{extract::DefaultBodyLimit, routing, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::server::actions::{action_of_ip::get_socket_handler, action_of_upload::upload_handler};

pub fn api_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/upload", routing::post(upload_handler))
        // ここはipを確認するだけですので、実際は使っていない
        .route("/socket-ip", routing::get(get_socket_handler))
        .layer(cors)
}

pub fn app() -> Router {
    Router::new()
        // .nest("/axum-api", ...) を使うと、URLが /axum-api で始まるリクエストはすべてこのネスト内部で完結します。
        .nest("/axum-api", api_router())
        // nestにもマッチしなかった場合に発動　静的ファイル(SPA)
        .fallback_service(ServeDir::new("./out"))
        // 1024 バイト = 1 KB (キロバイト)　1024 KB = 1 MB (メガバイト)　1024 MB = 1 GB (ギガバイト)
        // デフォルトのボディサイズ制限（2MB程度）を 1 GB（ギガバイト）に拡張
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
}

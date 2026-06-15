use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Request},
    http::{header, StatusCode},
    response::{Html, Response},
    routing, Router,
};
use rust_embed::RustEmbed;
use tower_http::cors::{Any, CorsLayer};

use crate::server::actions::{action_of_ip::get_socket_handler, action_of_upload::upload_handler};

/// コンパイル時に `../out` ディレクトリ（Next.js の SSG 出力）の全ファイルを
/// バイナリに埋め込む。これにより、Tauri プログラムは単一の実行ファイルになり、
/// 実行時に `./out` ディレクトリを参照する必要がなくなる。
#[derive(RustEmbed)]
#[folder = "../out"]
struct Assets;

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

/// 埋め込みアセットを取得し、`mime_guess` で適切な Content-Type を付与して返す
fn serve_embedded(path: &str) -> Option<Response> {
    let content = Assets::get(path)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    Some(
        Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(Body::from(content.data))
            .unwrap(),
    )
}

/// GET /upload で外部向けアップロードページ (upload.html) を返す
async fn serve_upload_page() -> Result<Html<String>, StatusCode> {
    Assets::get("upload.html")
        .map(|content| Html(String::from_utf8_lossy(&content.data).into_owned()))
        .ok_or(StatusCode::NOT_FOUND)
}

/// 静的ファイルのフォールバックハンドラ
/// API ルート（/axum-api）にも /upload にもマッチしなかったリクエストは
/// すべてここに到達し、埋め込みアセットから該当ファイルを探して返す。
async fn static_handler(request: Request) -> Response {
    let path = request.uri().path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    serve_embedded(path)
        .or_else(|| serve_embedded("404.html"))
        .unwrap_or_else(|| {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .unwrap()
        })
}

pub fn app() -> Router {
    Router::new()
        // .nest("/axum-api", ...) を使うと、URLが /axum-api で始まるリクエストはすべてこのネスト内部で完結します。
        .nest("/axum-api", api_router())
        // /upload の GET リクエストで外部向けアップロードページを返す
        .route("/upload", routing::get(serve_upload_page))
        // 上記の「API（/axum-api）」にも「アップロードページ（/upload）」にもマッチしなかったすべてのリクエストは、
        // 埋め込みアセット（rust-embed でバイナリに埋め込まれた ./out ディレクトリの内容）から該当ファイルを探して返す。
        // 例えば、/_next/static/... や /images/... へのアクセスは、コンパイル時に埋め込まれた JS/CSS/画像がそのまま返る。
        // .fallback_service(ServeDir::new("./out"))
        .fallback(static_handler)
        // 1024 バイト = 1 KB (キロバイト)　1024 KB = 1 MB (メガバイト)　1024 MB = 1 GB (ギガバイト)
        // デフォルトのボディサイズ制限（2MB程度）を 1 GB（ギガバイト）に拡張
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
}

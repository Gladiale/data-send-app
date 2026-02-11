mod io;
mod server;
use crate::{io::get_axum_port, server::router::api_router};
use std::net::SocketAddr;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("port: {}", get_axum_port());

    // --- ここが重要：実際のHTTPサーバーをバックグラウンドで立てる ---
    tokio::spawn(async move {
        // 0.0.0.0 は「すべてのネットワークインターフェース」で待機するという意味
        let addr = SocketAddr::from(([0, 0, 0, 0], get_axum_port()));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, api_router()).await.unwrap();
    });

    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        // .plugin(tauri_plugin_axum::init(app_router)) // Tauri内部用 （デスクアプリ）
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

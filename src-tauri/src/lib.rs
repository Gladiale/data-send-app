mod command;
mod io;
mod server;

use crate::{
    command::{get_socket_addr, open_folder},
    io::get_axum_port,
    server::router::app,
};
use std::net::SocketAddr;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // --- ここが重要：実際のHTTPサーバーをバックグラウンドで立てる ---
    tokio::spawn(async move {
        // 0.0.0.0 は「すべてのネットワークインターフェース」で待機するという意味
        let addr = SocketAddr::from(([0, 0, 0, 0], get_axum_port()));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app()).await.unwrap();
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
        // NEXT.JS側に公開 参照: https://v2.tauri.app/ja/develop/calling-rust/#basic-example
        .invoke_handler(tauri::generate_handler![get_socket_addr, open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

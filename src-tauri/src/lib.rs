mod io;
mod server;
// #[cfg(not(dev))]
use tauri::{ipc::CapabilityBuilder, Manager, Url};
use tauri::{WebviewUrl, WebviewWindowBuilder};

use crate::{
    io::{get_axum_port, get_tauri_port, init_local_ip},
    server::router::api_router,
};
use std::net::SocketAddr;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // println!("port: {}", get_axum_port());
    println!(
        "url: {}",
        format!("http://{}:{}", init_local_ip(), get_tauri_port())
    );

    // --- ここが重要：実際のHTTPサーバーをバックグラウンドで立てる ---
    tokio::spawn(async move {
        // 0.0.0.0 は「すべてのネットワークインターフェース」で待機するという意味
        let addr = SocketAddr::from(([0, 0, 0, 0], get_axum_port()));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, api_router()).await.unwrap();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new(get_tauri_port()).build())
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // tauri-plugin-localhost
            #[cfg(dev)]
            let url = WebviewUrl::App(std::path::PathBuf::from("/"));

            #[cfg(not(dev))]
            let url = {
                let url: Url = format!("http://{}:{}", init_local_ip(), get_tauri_port())
                    .parse()
                    .unwrap();

                app.add_capability(
                    CapabilityBuilder::new("localhost")
                        .remote(url.to_string()) // ここで指定したURLからのアクセスのみ許可
                        .window("main"),
                )?;

                WebviewUrl::External(url)
            };

            // This requires you to remove the window from tauri.conf.json
            WebviewWindowBuilder::new(app, "main".to_string(), url)
                .title("Localhost Example")
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

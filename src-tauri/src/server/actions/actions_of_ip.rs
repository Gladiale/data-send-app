use crate::io::{get_axum_port, get_tauri_port};
use axum::{http::StatusCode, response::IntoResponse, Json};
use local_ip_address::local_ip;
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct AddressResponse {
    // tauri内部のaxum-server
    axum_socket_addr: String,
    // tauriアプリケーションのlocal-server
    tauri_socket_addr: String,
}

// 関数の責務の分離
// Rustにおける SocketAddr とは、「IPアドレス」と「ポート番号」をセットにしたデータ型のことです。
fn build_socket_struct() -> Result<AddressResponse, String> {
    // ロカールIPを取得、取得失敗のエラーメッセージはResultに伝播
    let local_ip = local_ip().map_err(|e| format!("Failed to get local IP: {}", e))?;
    // SocketAddrを作成し、文字列に変換 (SocketAddr型だとJS側のシリアライズが問題になるので)
    let axum_socket_addr = SocketAddr::new(local_ip, get_axum_port()).to_string();
    let tauri_socket_addr = SocketAddr::new(local_ip, get_tauri_port()).to_string();

    Ok(AddressResponse {
        // 簡略記法
        axum_socket_addr,
        tauri_socket_addr,
    })
}

// Axumのハンドラ関数は「非同期（async）関数」であることが要求される
// Stringをそのまま返すのではなく、JSONとして返すのが一般的です
pub async fn get_socket_handler() -> impl IntoResponse {
    match build_socket_struct() {
        Ok(addr) => (StatusCode::OK, Json(addr)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
    }
}

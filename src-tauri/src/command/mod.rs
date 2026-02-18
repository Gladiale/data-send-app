use crate::io::get_axum_port;
use local_ip_address::local_ip;
use std::net::SocketAddr;

// Rustにおける SocketAddr とは、「IPアドレス」と「ポート番号」をセットにしたデータ型のことです。
#[tauri::command]
pub fn get_socket_addr() -> Result<String, String> {
    // ロカールIPを取得、取得失敗のエラーメッセージはResultに伝播
    let local_ip = local_ip().map_err(|e| format!("Failed to get local IP: {}", e))?;
    // SocketAddrを作成し、文字列に変換 (SocketAddr型だとJS側のシリアライズが問題になるので)
    let axum_socket_addr = SocketAddr::new(local_ip, get_axum_port()).to_string();

    Ok(axum_socket_addr)
}

use crate::io::get_axum_port;
use local_ip_address::local_ip;
use std::process::Command;
use std::{fs, net::SocketAddr};

// Rustにおける SocketAddr とは、「IPアドレス」と「ポート番号」をセットにしたデータ型のことです。
#[tauri::command]
pub fn get_socket_addr() -> Result<String, String> {
    // ロカールIPを取得、取得失敗のエラーメッセージはResultに伝播
    let local_ip = local_ip().map_err(|e| format!("Failed to get local IP: {}", e))?;
    // SocketAddrを作成し、文字列に変換 (SocketAddr型だとJS側のシリアライズが問題になるので)
    let axum_socket_addr = SocketAddr::new(local_ip, get_axum_port()).to_string();

    Ok(axum_socket_addr)
}

/*
    OSごとのコマンド:
        Windows: explorer コマンドを使用します。
        macOS: open コマンドを使用します。
        Linux: xdg-open を使うのが一般的です（デスクトップ環境に依存せず開けます）。

    .spawn(): プロセスの完了を待たずにバックグラウンドで実行します。フォルダを開くだけなら .status() で待機する必要はありません。
*/
#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    // 保存先ディレクトリがなければ作成
    fs::create_dir_all(&path).map_err(|e| format!("Failed to create directory: {e}"))?;

    #[cfg(target_os = "windows")]
    let output = {
        Command::new("explorer")
            // windowsは / をreplaceしないと正確なrouteを認識することができない
            .arg(path.replace("/", "\\"))
            .spawn()
    };

    #[cfg(target_os = "macos")]
    let output = { Command::new("open").arg(path).spawn() };

    #[cfg(target_os = "linux")]
    let output = { Command::new("xdg-open").arg(path).spawn() };

    output
        .map(|_| ())
        .map_err(|e| format!("フォルダを開けませんでした: {}", e))
}

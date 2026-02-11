use dotenv::dotenv;
use local_ip_address::local_ip;
use std::{env, sync::OnceLock};

// グローバルな保存場所を作る
static TAURI_PORT: OnceLock<u16> = OnceLock::new();

pub fn get_tauri_port() -> u16 {
    // まだ値が入っていなければ、ポートを取得して保存。あればそれを返す。
    *TAURI_PORT.get_or_init(|| portpicker::pick_unused_port().expect("failed to find unused port"))
}

// 環境変数からAXUMサーバーのPORT番号を取得
pub fn get_axum_port() -> u16 {
    // .envファイルを読み込む
    dotenv().ok();

    let port: u16 = env::var("NEXT_PUBLIC_AXUM_PORT")
        .ok() // ResultをOptionに変換
        .and_then(|p| p.parse().ok()) // パースに成功した時だけ値を上書き
        .unwrap_or(8080); // 失敗、または環境変数がない場合はデフォルト値

    port
}

pub fn init_local_ip() -> String {
    let ip: String = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(_) => "localhost".to_string(),
    };
    ip
}

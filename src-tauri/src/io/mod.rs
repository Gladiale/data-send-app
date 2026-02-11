use dotenv::dotenv;
use std::env;

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

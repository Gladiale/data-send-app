use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Serialize)]
struct FileResponse {
    file_name: String,
    uploaded: bool,
}

// 関数の責務の分離
async fn upload(mut multipart: Multipart) -> Result<Vec<FileResponse>, String> {
    let mut response: Vec<FileResponse> = Vec::new();
    let upload_dir = "./uploads";

    // 保存先ディレクトリがなければ作成
    fs::create_dir_all(upload_dir)
        .await
        .map_err(|e| format!("Failed to create directory: {e}"))?;

    // multipartを一つずつ取り出す
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| format!("Failed to get contents: {e}"))?
    {
        // フィールド名の確認
        if field.name() != Some("files") {
            continue;
        }

        // ファイル名の取得とサニタイズ（簡易的）
        let raw_file_name = field.file_name().unwrap_or("unknown_file").to_string();
        // セキュリティ対策：パス区切り文字を除去
        let safe_file_name =
            raw_file_name.replace(|c: char| c == '/' || c == '\\' || c == ' ', "_");

        let mut success = false;

        // データの読み込み 画像本体はfield.bytes()で取得します
        if let Ok(data) = field.bytes().await {
            let path = format!("{}/{}", upload_dir, safe_file_name);

            // ファイル作成と書き込み
            let res = async {
                let mut file = File::create(&path).await?;
                // 画像を保存 File構造体でストレージに画像を書き込んで完了となります
                file.write_all(&data).await?;
                // ターボフィッシュ構文
                Ok::<(), std::io::Error>(())
            }
            .await;

            if res.is_ok() {
                success = true
            }
        }

        response.push(FileResponse {
            file_name: safe_file_name,
            uploaded: success,
        });
    }

    if response.is_empty() {
        return Err("can't find file.".to_string());
    }

    Ok(response)
}

// Responseを作成
// Axumのハンドラ関数は「非同期（async）関数」であることが要求される
// upload_handler 自体は multipart の中身を書き換える必要がなく、単に upload_internal に所有権を渡す（Moveする）だけ
pub async fn upload_handler(multipart: Multipart) -> impl IntoResponse {
    match upload(multipart).await {
        Ok(vec) => (StatusCode::OK, Json(vec)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

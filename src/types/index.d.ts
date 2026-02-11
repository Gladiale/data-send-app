// レスポンスの型はバックエンド統一
type ResSocketAddr = {
  axum_socket_addr: string;
  tauri_socket_addr: string;
};

export { type ResSocketAddr };

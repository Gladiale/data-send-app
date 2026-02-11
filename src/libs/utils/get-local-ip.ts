import { type ResSocketAddr } from "@/types";

const getSocketIp = async (): Promise<string> => {
  // .envファイルから環境変数を取得
  const axumPort = process.env.NEXT_PUBLIC_AXUM_PORT;
  if (!axumPort) {
    throw new Error("NEXT_PUBLIC_AXUM_PORT is not defined.");
  }

  try {
    const response = await fetch(`http://localhost:${axumPort}/axum-api/socket-ip`);

    // Http StatusCode チェック
    if (!response.ok) {
      // バックエンドからはstringでエラーメッセージを送ってくるので、ここは.text()を使う
      const err = await response.text();
      throw new Error(err);
    }

    const resData = (await response.json()) as ResSocketAddr;
    return resData.socket_addr;
  } catch (error) {
    // 再 throw して呼び出し側に例外を伝える
    if (error instanceof Error) throw error;
    throw new Error("フェッチが失敗しました！");
  }
};

export { getSocketIp };

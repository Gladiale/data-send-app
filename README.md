### 参照
https://zenn.dev/kei1232/articles/d171d6f9c3aa81  
https://tkwork.tech/tauri-nextjs-setup-tutorial/

### 公式
https://v2.tauri.app/ja/start/frontend/nextjs/  
https://v2.tauri.app/ja/develop/calling-rust/#basic-example  
https://v2.tauri.app/ja/plugin/notification/

### cli
https://v2.tauri.app/ja/reference/cli/

```bash
# nextjs set-up
npx create-next-app@latest

# tauri-cli をインストール
cd app-name
npm install --save-dev @tauri-apps/cli@latest

# tauriを初期化
npx tauri init

# 色々設定を行う
https://zenn.dev/kei1232/articles/d171d6f9c3aa81
https://v2.tauri.app/ja/start/frontend/nextjs/
https://tkwork.tech/tauri-nextjs-setup-tutorial/

# invokeでフロントからRustのコードを実行する
npm install @tauri-apps/api
```
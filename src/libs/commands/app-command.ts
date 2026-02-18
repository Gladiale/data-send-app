import { invoke } from "@tauri-apps/api/core";

export class AppCommand {
  static async getSocketAddr(): Promise<string> {
    return await invoke("get_socket_addr");
  }

  static async openFolder(path: string) {
    await invoke("open_folder", { path });
  }
}

import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

interface RecentPathsState {
  paths: string[];
  error: string | null;
}

function createRecentPathsStore() {
  const { subscribe, set, update } = writable<RecentPathsState>({
    paths: [],
    error: null,
  });

  return {
    subscribe,

    async load() {
      try {
        const paths = await invoke<string[]>("get_recent_paths");
        set({ paths, error: null });
      } catch (e) {
        update((state) => ({ ...state, error: String(e) }));
      }
    },

    /** 開いたパスを履歴の先頭に記録する。呼び出し側で loadFrom 成功後に呼ぶこと。 */
    async record(path: string) {
      try {
        await invoke("add_recent_path", { path });
        await this.load();
      } catch (e) {
        update((state) => ({ ...state, error: String(e) }));
      }
    },
  };
}

export const recentPathsStore = createRecentPathsStore();

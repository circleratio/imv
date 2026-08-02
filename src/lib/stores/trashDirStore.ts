import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

interface TrashDirState {
  path: string | null;
  error: string | null;
}

function createTrashDirStore() {
  const { subscribe, set, update } = writable<TrashDirState>({
    path: null,
    error: null,
  });

  return {
    subscribe,

    async load() {
      try {
        const path = await invoke<string>("get_trash_dir");
        set({ path, error: null });
      } catch (e) {
        update((state) => ({ ...state, error: String(e) }));
      }
    },

    async set(path: string) {
      try {
        await invoke("set_trash_dir", { path });
        set({ path, error: null });
      } catch (e) {
        update((state) => ({ ...state, error: String(e) }));
      }
    },
  };
}

export const trashDirStore = createTrashDirStore();

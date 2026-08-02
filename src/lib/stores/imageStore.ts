import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

export type ViewState = "picking" | "viewing" | "empty";

interface ImageState {
  view: ViewState;
  files: string[];
  history: string[];
  position: number;
  current: string | null;
  error: string | null;
}

function pickRandom(files: string[]): string | null {
  if (files.length === 0) return null;
  const index = Math.floor(Math.random() * files.length);
  return files[index];
}

const emptyState: ImageState = {
  view: "empty",
  files: [],
  history: [],
  position: -1,
  current: null,
  error: null,
};

function createImageStore() {
  const { subscribe, update, set } = writable<ImageState>({
    view: "picking",
    files: [],
    history: [],
    position: -1,
    current: null,
    error: null,
  });

  /** historyBase の末尾に新しいランダム画像を積んで表示する。 */
  function pushRandom(state: ImageState, historyBase: string[]): ImageState {
    const next = pickRandom(state.files);
    if (next === null) {
      return { ...emptyState };
    }
    const history = [...historyBase, next];
    return {
      ...state,
      view: "viewing",
      history,
      position: history.length - 1,
      current: next,
      error: null,
    };
  }

  return {
    subscribe,

    /** scan_images が成功したら true を返す（呼び出し元は最近使ったパスの記録などに使える）。 */
    async loadFrom(path: string): Promise<boolean> {
      try {
        const files = await invoke<string[]>("scan_images", { path });
        if (files.length === 0) {
          set({ ...emptyState });
          return true;
        }
        const first = pickRandom(files);
        set({
          view: "viewing",
          files,
          history: first === null ? [] : [first],
          position: 0,
          current: first,
          error: null,
        });
        return true;
      } catch (e) {
        set({
          view: "picking",
          files: [],
          history: [],
          position: -1,
          current: null,
          error: String(e),
        });
        return false;
      }
    },

    next() {
      update((state) => pushRandom(state, state.history.slice(0, state.position + 1)));
    },

    prev() {
      update((state) => {
        if (state.position <= 0) return state;
        const position = state.position - 1;
        return { ...state, position, current: state.history[position], error: null };
      });
    },

    async deleteCurrent() {
      let target: string | null = null;
      update((state) => {
        target = state.current;
        return state;
      });
      if (!target) return;

      try {
        await invoke<string>("move_to_trash", { path: target });
        update((state) => {
          const files = state.files.filter((f) => f !== target);
          // 削除された画像は履歴からも取り除く（それ以降の先読み履歴も破棄する）
          const historyBase = state.history.slice(0, state.position);
          return pushRandom({ ...state, files }, historyBase);
        });
      } catch (e) {
        update((state) => ({ ...state, error: String(e) }));
      }
    },

    reset() {
      set({ view: "picking", files: [], history: [], position: -1, current: null, error: null });
    },
  };
}

export const imageStore = createImageStore();

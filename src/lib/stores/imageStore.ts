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
  /** Deleted files that can be restored, most recent last. */
  undoStack: TrashedEntry[];
}

interface TrashedEntry {
  original: string;
  trashed: string;
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
  undoStack: [],
};

function createImageStore() {
  const { subscribe, update, set } = writable<ImageState>({
    view: "picking",
    files: [],
    history: [],
    position: -1,
    current: null,
    error: null,
    undoStack: [],
  });

  /** historyBase の末尾に新しいランダム画像を積んで表示する。 */
  function pushRandom(state: ImageState, historyBase: string[]): ImageState {
    const next = pickRandom(state.files);
    if (next === null) {
      return { ...emptyState, undoStack: state.undoStack };
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
          undoStack: [],
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
          undoStack: [],
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
        const trashed = await invoke<string>("move_to_trash", { path: target });
        update((state) => {
          const files = state.files.filter((f) => f !== target);
          const undoStack = [...state.undoStack, { original: target!, trashed }];
          // 削除された画像は履歴からも取り除く（それ以降の先読み履歴も破棄する）
          const historyBase = state.history.slice(0, state.position);
          return pushRandom({ ...state, files, undoStack }, historyBase);
        });
      } catch (e) {
        update((state) => ({ ...state, error: String(e) }));
      }
    },

    /** 直近の削除を取り消し、復元した画像を表示する。 */
    async undoDelete() {
      let entry: TrashedEntry | undefined;
      update((state) => {
        entry = state.undoStack[state.undoStack.length - 1];
        return state;
      });
      if (!entry) return;
      const { original, trashed } = entry;

      try {
        await invoke<string>("restore_from_trash", {
          trashedPath: trashed,
          originalPath: original,
        });
        update((state) => {
          const undoStack = state.undoStack.slice(0, -1);
          const files = state.files.includes(original) ? state.files : [...state.files, original];
          // Show the restored image as the newest history entry (discarding forward history)
          const history = [...state.history.slice(0, state.position + 1), original];
          return {
            ...state,
            view: "viewing",
            files,
            history,
            position: history.length - 1,
            current: original,
            error: null,
            undoStack,
          };
        });
      } catch (e) {
        update((state) => ({ ...state, error: String(e) }));
      }
    },

    reset() {
      set({
        view: "picking",
        files: [],
        history: [],
        position: -1,
        current: null,
        error: null,
        undoStack: [],
      });
    },
  };
}

export const imageStore = createImageStore();

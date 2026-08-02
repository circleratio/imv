<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";

  let {
    canOperate,
    canGoPrev,
    remaining,
    recentPaths,
    onPick,
    onPrev,
    onNext,
    onDelete,
    onOpenSettings,
  }: {
    canOperate: boolean;
    canGoPrev: boolean;
    remaining: number;
    recentPaths: string[];
    onPick: (path: string) => void;
    onPrev: () => void;
    onNext: () => void;
    onDelete: () => void;
    onOpenSettings: () => void;
  } = $props();

  let showOpenMenu = $state(false);
  let showHistoryMenu = $state(false);

  function toggleOpenMenu(event: MouseEvent) {
    event.stopPropagation();
    showHistoryMenu = false;
    showOpenMenu = !showOpenMenu;
  }

  function toggleHistoryMenu(event: MouseEvent) {
    event.stopPropagation();
    showOpenMenu = false;
    showHistoryMenu = !showHistoryMenu;
  }

  function closeMenus() {
    showOpenMenu = false;
    showHistoryMenu = false;
  }

  async function pickFolder() {
    showOpenMenu = false;
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") onPick(selected);
  }

  async function pickFile() {
    showOpenMenu = false;
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [
        {
          name: "画像",
          extensions: ["jpg", "jpeg", "png", "gif", "bmp", "webp"],
        },
      ],
    });
    if (typeof selected === "string") onPick(selected);
  }

  function pickFromHistory(path: string) {
    showHistoryMenu = false;
    onPick(path);
  }
</script>

<svelte:window onclick={closeMenus} />

<div class="command-bar">
  <div class="group">
    <div class="dropdown">
      <button type="button" onclick={toggleOpenMenu}>開く ▾</button>
      {#if showOpenMenu}
        <div
          class="menu"
          onclick={(e) => e.stopPropagation()}
          onkeydown={(e) => e.key === "Escape" && (showOpenMenu = false)}
          role="menu"
          tabindex="-1"
        >
          <button type="button" role="menuitem" onclick={pickFolder}>フォルダを選択</button>
          <button type="button" role="menuitem" onclick={pickFile}>ファイルを選択</button>
        </div>
      {/if}
    </div>

    <div class="dropdown">
      <button type="button" onclick={toggleHistoryMenu}>履歴 ▾</button>
      {#if showHistoryMenu}
        <div
          class="menu history-menu"
          onclick={(e) => e.stopPropagation()}
          onkeydown={(e) => e.key === "Escape" && (showHistoryMenu = false)}
          role="menu"
          tabindex="-1"
        >
          {#if recentPaths.length === 0}
            <span class="empty-item">履歴はありません</span>
          {:else}
            {#each recentPaths as path (path)}
              <button type="button" role="menuitem" title={path} onclick={() => pickFromHistory(path)}>
                {path}
              </button>
            {/each}
          {/if}
        </div>
      {/if}
    </div>

    <button type="button" disabled={!canGoPrev} onclick={onPrev}>前へ</button>
    <button type="button" disabled={!canOperate} onclick={onNext}>次へ</button>
    <button type="button" disabled={!canOperate} onclick={onDelete}>削除</button>
    {#if canOperate}
      <span class="count">残り {remaining} 枚</span>
    {/if}
  </div>
  <button type="button" class="settings" onclick={onOpenSettings}>設定</button>
</div>

<style>
  .command-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.6em 1em;
    background-color: #202020;
    color: #f0f0f0;
    gap: 1em;
  }

  .group {
    display: flex;
    align-items: center;
    gap: 0.5em;
  }

  .dropdown {
    position: relative;
  }

  .menu {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 0.25em;
    background-color: #2b2b2b;
    border-radius: 6px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    min-width: 10em;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    z-index: 20;
  }

  .history-menu {
    max-width: 28em;
    max-height: 20em;
    overflow-y: auto;
  }

  .menu button {
    text-align: left;
    background: none;
    border-radius: 0;
    padding: 0.6em 0.9em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .menu button:hover {
    background-color: #3a3a3a;
  }

  .empty-item {
    padding: 0.6em 0.9em;
    color: #888888;
    font-size: 0.9em;
  }

  button {
    border-radius: 6px;
    border: 1px solid transparent;
    padding: 0.45em 1em;
    font-size: 0.95em;
    font-weight: 500;
    cursor: pointer;
    background-color: #396cd8;
    color: #ffffff;
  }

  button:hover:not(:disabled) {
    background-color: #2f5bb8;
  }

  button:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .settings {
    background-color: #444444;
  }

  .settings:hover {
    background-color: #555555;
  }

  .count {
    margin-left: 0.5em;
    font-size: 0.85em;
    color: #b0b0b0;
  }
</style>

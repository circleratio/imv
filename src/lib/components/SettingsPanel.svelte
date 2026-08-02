<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { trashDirStore } from "$lib/stores/trashDirStore";

  let { onClose }: { onClose: () => void } = $props();

  async function changeTrashDir() {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") {
      await trashDirStore.set(selected);
    }
  }
</script>

<div class="overlay" onclick={onClose} role="presentation">
  <div
    class="panel"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.key === "Escape" && onClose()}
    role="dialog"
    aria-label="設定"
    tabindex="-1"
  >
    <h2>設定</h2>

    <div class="field">
      <span class="label">削除した画像の移動先</span>
      <span class="value">{$trashDirStore.path ?? "読み込み中..."}</span>
      <button type="button" onclick={changeTrashDir}>変更</button>
    </div>

    {#if $trashDirStore.error}
      <p class="error">{$trashDirStore.error}</p>
    {/if}

    <button type="button" class="close" onclick={onClose}>閉じる</button>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
  }

  .panel {
    background-color: #202020;
    color: #f0f0f0;
    border-radius: 8px;
    padding: 1.5em;
    min-width: 24em;
    max-width: 90vw;
  }

  h2 {
    margin-top: 0;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.4em;
    margin-bottom: 1em;
  }

  .label {
    font-size: 0.85em;
    color: #b0b0b0;
  }

  .value {
    word-break: break-all;
    background-color: #151515;
    border-radius: 6px;
    padding: 0.5em 0.7em;
  }

  button {
    align-self: flex-start;
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.5em 1.2em;
    font-size: 0.95em;
    font-weight: 500;
    cursor: pointer;
    background-color: #396cd8;
    color: #ffffff;
  }

  button:hover {
    background-color: #2f5bb8;
  }

  .close {
    margin-top: 0.5em;
    background-color: #444444;
  }

  .close:hover {
    background-color: #555555;
  }

  .error {
    color: #ff6b6b;
  }
</style>

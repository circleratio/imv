<script lang="ts">
  import { onMount } from "svelte";
  import { imageStore } from "$lib/stores/imageStore";
  import { trashDirStore } from "$lib/stores/trashDirStore";
  import { recentPathsStore } from "$lib/stores/recentPathsStore";
  import CommandBar from "$lib/components/CommandBar.svelte";
  import PickerView from "$lib/components/PickerView.svelte";
  import ImageViewer from "$lib/components/ImageViewer.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import SettingsPanel from "$lib/components/SettingsPanel.svelte";

  let showSettings = $state(false);

  onMount(() => {
    trashDirStore.load();
    recentPathsStore.load();
  });

  async function handlePick(path: string) {
    const ok = await imageStore.loadFrom(path);
    if (ok) recentPathsStore.record(path);
  }

  function handleKeydown(event: KeyboardEvent) {
    // Ignore shortcuts while typing in a text field or when the settings panel is open
    const target = event.target as HTMLElement | null;
    if (
      showSettings ||
      target?.closest("input, textarea, select, [contenteditable]")
    ) {
      return;
    }
    const plainKey = !(event.ctrlKey || event.altKey || event.metaKey);
    // Undo is also available from the empty state (e.g. after deleting the last image)
    if ((event.key === "u" || event.key === "U") && plainKey) {
      if ($imageStore.view === "viewing" || $imageStore.view === "empty") {
        event.preventDefault();
        imageStore.undoDelete();
      }
      return;
    }
    if ($imageStore.view !== "viewing") return;
    if (event.key === "d" || event.key === "D") {
      // Plain "d" only; leave modifier combinations (e.g. Ctrl+D) alone
      if (!plainKey) return;
      event.preventDefault();
      imageStore.deleteCurrent();
    } else if (event.key === "ArrowRight" || event.key === " ") {
      event.preventDefault();
      imageStore.next();
    } else if (event.key === "ArrowLeft") {
      event.preventDefault();
      imageStore.prev();
    } else if (event.key === "Delete") {
      event.preventDefault();
      imageStore.deleteCurrent();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="app">
  <CommandBar
    canOperate={$imageStore.view === "viewing"}
    canGoPrev={$imageStore.view === "viewing" && $imageStore.position > 0}
    remaining={$imageStore.files.length}
    recentPaths={$recentPathsStore.paths}
    onPick={handlePick}
    onPrev={() => imageStore.prev()}
    onNext={() => imageStore.next()}
    onDelete={() => imageStore.deleteCurrent()}
    onOpenSettings={() => (showSettings = true)}
  />

  <div class="content">
    {#if $imageStore.view === "picking"}
      <PickerView />
    {:else if $imageStore.view === "viewing" && $imageStore.current}
      <ImageViewer path={$imageStore.current} />
    {:else if $imageStore.view === "empty"}
      <EmptyState />
    {/if}
  </div>

  {#if $imageStore.error}
    <p class="toast">{$imageStore.error}</p>
  {/if}

  {#if showSettings}
    <SettingsPanel onClose={() => (showSettings = false)} />
  {/if}
</main>

<style>
  :global(html, body) {
    margin: 0;
    height: 100%;
  }

  :global(#svelte) {
    height: 100%;
  }

  .app {
    position: relative;
    display: flex;
    flex-direction: column;
    height: 100vh;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }

  .content {
    flex: 1;
    display: flex;
    min-height: 0;
  }

  .toast {
    position: absolute;
    bottom: 1.5em;
    left: 50%;
    transform: translateX(-50%);
    background-color: #ff6b6b;
    color: #ffffff;
    padding: 0.5em 1em;
    border-radius: 6px;
  }
</style>

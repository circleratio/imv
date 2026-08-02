<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";

  let { path }: { path: string } = $props();

  let src = $derived(convertFileSrc(path));
</script>

<div class="viewer">
  {#key path}
    <img {src} alt={path} />
  {/key}
</div>

<style>
  .viewer {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 0;
    background-color: #1a1a1a;
  }

  img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }
</style>

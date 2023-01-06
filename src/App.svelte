<script lang="ts">
  import FileTreeView from "./lib/FileTreeView.svelte";
  import SelectFolder from "./lib/SelectFolder.svelte";
  import { folder } from "./lib/store";
  import ThemeSelector from "./lib/ThemeSelector.svelte";
</script>

<main class="container">
  <ThemeSelector />
  <SelectFolder />
  <FileTreeView />
</main>

<style>
</style>

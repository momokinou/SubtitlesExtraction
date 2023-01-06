<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { Button } from "carbon-components-svelte";
  import { folder } from "./store";

  async function setFolder() {
    try {
      let dossier = await open({
        multiple: false,
        title: "Open Text File",
        directory: true, // Delete if i want to select files
      });
      folder.set(dossier.toString());
    } catch (err) {
      console.log(err);
    }
  }
</script>

<div>
  <Button on:click={setFolder}>Open File Explorer</Button>
</div>

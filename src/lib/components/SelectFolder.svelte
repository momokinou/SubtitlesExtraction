<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button } from "carbon-components-svelte";
  import { folder, files } from "./store";

  async function setFolder() {
    try {
      let dossier = await open({
        multiple: false,
        title: "Open Text File",
        directory: true, // Delete if i want to select files
      });
      folder.set(dossier.toString());
      let files_list: [] = await invoke("list_files", { dir: dossier });
      files.set(files_list);
    } catch (err) {
      console.log(err);
    }
  }
</script>

<div>
  <Button on:click={setFolder}>Open File Explorer</Button>
</div>

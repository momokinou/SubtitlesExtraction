<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { Button } from "carbon-components-svelte";
  import { output_folder } from "../lib/components/store";

  let selectedFolder: string;

  output_folder.subscribe((value) => {
    selectedFolder = value;
  });

  async function setFolder() {
    try {
      let dossier = await open({
        multiple: false,
        title: "Open Text File",
        directory: true, // Delete if i want to select files
      });
      output_folder.set(dossier.toString());
    } catch (err) {
      console.log(err);
    }
  }
</script>

<div>
  <Button on:click={setFolder}>Select output Folder</Button>
  <p>Selected Folder: {selectedFolder}</p>
</div>

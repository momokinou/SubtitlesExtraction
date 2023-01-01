<script lang="ts">
    import { open } from "@tauri-apps/api/dialog"
    import { Button } from "carbon-components-svelte";
    import { invoke } from '@tauri-apps/api/tauri'
    const readFile = async () => {
        try {
            let dossier = await open({
                multiple: false,
                title: "Open Text File",
                directory: true, // Delete if i want to select files
        });
        // console.log(await invoke("liste_fichiers", {selectedPath})
        console.log(await invoke("liste_fichiers", {dossier}));
        } catch(err) {
            console.log(err)
        }
    };
</script>

<div>
    <Button on:click={readFile}>Open File Explorer</Button>
</div>
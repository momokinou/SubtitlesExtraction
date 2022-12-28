<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { open } from "@tauri-apps/api/dialog"

    const readFile =async () => {
        try {
            const selectedPath = await open({
                multiple: false,
                title: "Open Text File",
                directory: true, // Delete if i want to select files
        });
        console.log(selectedPath);
        } catch(err) {
            console.log(err)
        }
    };
</script>
<div>
    <button on:click={readFile}>Open File Explorer</button>
</div>
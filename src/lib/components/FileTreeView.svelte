<script lang="ts">
  import { RadioButtonSkeleton, TreeView } from "carbon-components-svelte";
  import { onDestroy } from "svelte";
  import { folder } from "./store";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { TreeNode } from "carbon-components-svelte/types/TreeView/TreeView.svelte";

  let children: TreeNode[] = [];
  let dossier: string;
  let files: string[];
  const unsubscribeFolder = folder.subscribe(async (value) => {
    if (value != "null") {
      dossier = value.split("\\").pop();
      let truc: TreeNode[] = await invoke("list_subdirectories", {
        dir: value,
        id: 1,
      });
      children = JSON.parse(truc);

      files = await invoke("list_files", {
        dir: value,
      });
    }
  });

  onDestroy(unsubscribeFolder);

  const callRust = () => {
    invoke("analyze", { files });
  };
</script>

{#key dossier}
  <TreeView labelText={dossier} {children} />
{/key}
{#key files}
  <RadioButtonSkeleton on:click={callRust()} />
{/key}

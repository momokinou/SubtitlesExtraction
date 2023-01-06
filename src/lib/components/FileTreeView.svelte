<script lang="ts">
  import { TreeView } from "carbon-components-svelte";
  import { onDestroy } from "svelte";
  import { folder } from "./store";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { TreeNode } from "carbon-components-svelte/types/TreeView/TreeView.svelte";

  let children: TreeNode[] = [];
  let dossier: string;

  const unsubscribeFolder = folder.subscribe(async (value) => {
    dossier = value.split("\\").pop();
    let truc: TreeNode[] = await invoke("list_subdirectories", {
      dir: value,
      id: 1,
    });
    children = JSON.parse(truc);
  });

  onDestroy(unsubscribeFolder);
</script>

{#key dossier}
  <TreeView labelText={dossier} {children} />
{/key}

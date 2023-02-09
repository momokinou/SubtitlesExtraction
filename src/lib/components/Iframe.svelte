<script lang="ts">
  export let app;
  export let url;
  import { onMount, afterUpdate } from "svelte";

  onMount(() => {
    console.log(`Mounted ${app}`);
  });

  afterUpdate(() => {
    console.log(`Updated ${app}`);
  });
</script>

<iframe title={app} id="microapp-{app}" frameborder="0" src={url} />

<style>
  iframe {
    width: 100%;
    height: 1000px;
  }
</style>

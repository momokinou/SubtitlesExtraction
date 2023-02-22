<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    Theme,
    RadioButtonGroup,
    RadioButton,
    RadioButtonSkeleton,
  } from "carbon-components-svelte";

  let theme = "white";

  const callRust = () => {
    invoke("analyze");
  };
</script>

<Theme bind:theme persist persistKey="carbon_theme" />
<RadioButtonGroup legendText="Carbon theme" bind:selected={theme}>
  {#each ["white", "g10", "g80", "g90", "g100"] as value}
    <RadioButton labelText={value} {value} />
  {/each}
</RadioButtonGroup>
<RadioButtonSkeleton on:click={callRust()} />

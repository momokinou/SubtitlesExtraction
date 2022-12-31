<script lang="ts">
    import { onMount } from 'svelte'
    import { themeChange } from 'theme-change'
    import ThemeSelect from './lib/ThemeSelect.svelte'
    import './style.css'
    onMount(() => {
      themeChange(false)
    })
</script>


<main class="container">
  <ThemeSelect />
  <p>Here's some DaisyUI buttons, yummy!</p>

  <button class="btn">neutral</button>
  <button class="btn btn-primary">primary</button>
  <button class="btn btn-secondary">secondary</button>
  <button class="btn btn-accent">accent</button>
  <button class="btn btn-ghost">ghost</button>
  <button class="btn btn-link">link</button>
</main>

<style global lang="postcss">
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
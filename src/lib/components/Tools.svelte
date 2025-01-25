<script lang="ts">
  import { save, load } from "$lib/helper/save.svelte"
</script>

<section class="bg-purple-500 col-start-4 row-start-2 grid grid-cols-2 grid-rows-4 place-items-center">
    <h2 class="text-center col-span-2">Save & Load</h2>
    <button onclick={save} class="row-start-2">Save Game</button>
    <button onclick={load} class="row-start-2">Load Game</button>
    <button class="w-full h-full border-t-2 border-black col-span-2 row-span-2">Automation</button>
</section>

<script lang="ts">
  import "../app.css"

  import Battlezone from "$lib/components/Battlezone.svelte"
  import MonsterManagement from "$lib/components/MonsterManagement.svelte"
  import Shop from "$lib/components/Shop.svelte"

  import { save, load } from "$lib/helper/save.svelte"

</script>

<main class="h-screen p-4 bg-black grid gap-4 grid-cols-6 grid-rows-4">
  <Shop />
  <section class="bg-blue-500 col-span-2 col-start-3">Gold: {gold.getGold()}</section>
  <section class="bg-yellow-500 col-start-3 row-start-2">Prestige</section>
  <section class={`col-start-4 row-start-2 bg-purple-500`}>
    <button onclick={save}>Save Game</button>
    <button onclick={load}>Load Game</button>
  </section>
  <MonsterManagement />
  <Battlezone />
</main>

<style>
:root(html) {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
</style>

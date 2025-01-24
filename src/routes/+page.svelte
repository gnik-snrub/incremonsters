<script lang="ts">
  import "../app.css"

  import Battlezone from "$lib/components/Battlezone.svelte"
  import MonsterManagement from "$lib/components/MonsterManagement.svelte"
  import Shop from "$lib/components/Shop.svelte"
  import Prestige from "$lib/components/Prestige.svelte"
  import Tools from "$lib/components/Tools.svelte"
  import Resources from "$lib/components/Resources.svelte"

</script>

<main class="h-screen p-4 bg-black grid gap-4 grid-cols-6 grid-rows-4">
  <Shop />
  <Resources />
  <Prestige />
  <Tools />
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

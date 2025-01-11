<script lang="ts">
  import "../app.css"

  import {invoke} from "@tauri-apps/api/core";

  import Battlezone from "$lib/components/Battlezone.svelte"
  import MonsterManagement from "$lib/components/MonsterManagement.svelte"
  import Shop from "$lib/components/Shop.svelte"

  import { playerSquad, stable, enemySquad } from "../stores/monsters.svelte"
  import { gold } from "../stores/resources.svelte"
  import { atkBoost, defBoost, spdBoost } from "../stores/shop.svelte"
  import { battle } from "../stores/battleState.svelte"

  async function save() {
    const saveData = {
      monsterData: {
        player_squad: playerSquad.getMonsters(),
        stable: stable.getMonsters(),
        enemy_squad: enemySquad.getMonsters(),
      },
      resourceData: {
        gold: gold.getGold(),
      },
      shopData: {
        atk_boost: atkBoost.amountBought(),
        def_boost: defBoost.amountBought(),
        spd_boost: spdBoost.amountBought(),
      },
    }
    console.log(saveData)
    await invoke("save", { data: JSON.stringify(saveData) })

      /*
    try {
      await writeTextFile('test.json', 'Hello!', {baseDir: BaseDirectory.Home})

      await writeTextFile('things/incremonsters.json', JSON.stringify(saveData), {baseDir: BaseDirectory.Desktop})
    } catch (error) {
      console.error("Error: ", error)
    }
    */
  }

  async function load() {
    if (battle.isBattling()) battle.battleToggle()
    const saveData: string = await invoke("load")
    console.log(JSON.parse(saveData))
    stable.setMonsters(JSON.parse(saveData).monsterData.stable)
    playerSquad.setMonsters(JSON.parse(saveData).monsterData.player_squad)
    enemySquad.setMonsters(JSON.parse(saveData).monsterData.enemy_squad)

    gold.subtractGold(gold.getGold())
    gold.addGold(JSON.parse(saveData).resourceData.gold)

    atkBoost.reset()
    defBoost.reset()
    spdBoost.reset()

    for (let i = 0; i < JSON.parse(saveData).shopData.atk_boost; i++) {
      atkBoost.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).shopData.def_boost; i++) {
      defBoost.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).shopData.spd_boost; i++) {
      spdBoost.buy()
    }
  }
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

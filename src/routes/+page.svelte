<script lang="ts">
  import "../app.css"
  import { invoke } from "@tauri-apps/api/core"
  import { type Monster, isMonster } from "$lib/types/monster"

  import { playerSquad, enemySquad, stable } from "../stores/monsters.svelte";
  import { gold } from "../stores/resources.svelte";

  let isBattling: boolean = $state(false)
  let battleInterval: number = 1000
  let battleIntervalId: ReturnType<typeof setInterval>

  function battleToggle() {
    if (isBattling) {
      clearInterval(battleIntervalId)
      isBattling = false
    } else {
      battleIntervalId = setInterval(() => {
        invokeBattle()
      }, battleInterval)
      isBattling = true
    }
  }

  $effect(() => {
    // Just for testing...
    playerSquad.setMonsters([])
    for (let i = 0; i < 4; i++) {
      invoke("create_monster", { lvl: 1 }).then((res) => {
        if (isMonster(res)) {
          playerSquad.setMonsters([...playerSquad.getMonsters(), res])
        }
      })
    }
  })

  $effect(() => {
    enemySquad.newMonsters(dungeonLvl, 4)
  })

  let dungeonLvl: number = 1
  let pendingDungeonLevelIncrease: boolean = $state(false)
  let battleAutomation: boolean = false

  async function handleBattleWin(updatedPlayerMonsters: Monster[], goldEarned: number) {
    playerSquad.setMonsters(updatedPlayerMonsters)
    gold.addGold(goldEarned)

    const tamedMonsterIdx: number = Math.random() * (enemySquad.getMonsters().length - 1) | 0
    const tamedMonster: Monster = enemySquad.getMonsters()[tamedMonsterIdx]
    stable.setMonsters([...stable.getMonsters(), tamedMonster])

    pendingDungeonLevelIncrease = true

    if (battleAutomation) {
      await nextDungeon()
    }
  }

  async function nextDungeon() {
    if (pendingDungeonLevelIncrease) {
      dungeonLvl += 1
      await enemySquad.newMonsters(dungeonLvl, 4)
      battleToggle()
      pendingDungeonLevelIncrease = false
    }
  }

  let calculatingRewards: boolean = false

  $effect(async () => {
    if (isBattling && enemySquad.getAllDead() && !calculatingRewards && !pendingDungeonLevelIncrease) {
      calculatingRewards = true
      battleToggle()

      const response = await invoke("win_battle_rewards", { dungeonLvl, player: playerSquad.getMonsters(), enemy: enemySquad.getMonsters() })
      await handleBattleWin(response[0], response[1])
      calculatingRewards = false
    }
  })

  $effect(() => {
    if (isBattling && playerSquad.getAllDead()) {
      dungeonLvl = 1
      reset()
    }
  })

  function reset() {
    clearInterval(battleIntervalId)
    isBattling = false
    playerSquad.heal()
  }

  function invokeBattle() {
    invoke("battle", { player: playerSquad.getMonsters(), enemy: enemySquad.getMonsters() })
      .then((res) => {
        const newPlayer: Monster[] = res[0]
        const newEnemy: Monster[] = res[1]
        playerSquad.setMonsters(newPlayer)
        enemySquad.setMonsters(newEnemy)
        playerSquad.heal()
      })
  }

</script>

<main class="h-screen p-4 bg-black grid gap-4 grid-cols-6 grid-rows-4">
  <section class="bg-orange-500 col-span-2 row-span-2">Buyables</section>
  <section class="bg-blue-500 col-span-2 col-start-3">Resources</section>
  <section class="bg-yellow-500 col-start-3 row-start-2">Prestige</section>
  <section class={`col-start-4 row-start-2 ${pendingDungeonLevelIncrease ? 'bg-purple-200' : 'bg-purple-500'}`} onclick={nextDungeon}>
      {pendingDungeonLevelIncrease ? "Next dungeon!" : "Automation"}</section>
  <section class="h-full p-4 overflow-y-auto bg-red-500 col-span-2 row-span-2 col-start-5 row-start-1 grid grid-cols-4 gap-4 auto-rows-[25%] place-items-center">
    {#each stable.getMonsters() as monster}
      <div class="w-full h-full border-4 rounded-full border-rose-800 bg-rose-400 grid place-items-center max-w-24 max-h-24">
        <h3>{monster.name}</h3>
      </div>
    {/each}
  </section>
  <section class="flex flex-col w-full bg-green-500 align-center row-span-2 col-span-6 row-start-3">
    <div class="flex flex-row justify-center w-full gap-10">
      <button onclick={battleToggle}>Fight!</button>
      <button onclick={reset}>Reset</button>
    </div>
    <div class="flex flex-row self-center w-3/5 pt-8 border-t-4 border-black justify-evenly gap-4">
      {#each enemySquad.getMonsters() as monster }
        <div class="border-black">
          <h3>{monster.name}</h3>
          <p>Level: {monster.lvl}</p>
          <p>HP: {monster.current_hp}/{monster.hp}</p>
          <p>ATK: {monster.atk}</p>
          <p>DEF: {monster.def}</p>
          <p>SPD: {monster.spd}</p>
        </div>
      {/each}
    </div>
    <div class="flex flex-row self-center w-3/5 pt-8 mt-8 border-t-4 border-black justify-evenly gap-4">
      {#each playerSquad.getMonsters() as monster }
        <div>
          <h3>{monster.name}</h3>
          <p>Level: {monster.lvl}</p>
          <p>EXP: {monster.exp}</p>
          <p>HP: {monster.current_hp}/{monster.hp}</p>
          <p>ATK: {monster.atk}</p>
          <p>DEF: {monster.def}</p>
          <p>SPD: {monster.spd}</p>
        </div>
      {/each}
    </div>
  </section>
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

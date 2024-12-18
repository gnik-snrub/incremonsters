<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  import { type Monster, isMonster } from "$lib/types/monster"

  import { mySquad, enemySquad } from "../stores/monsters.svelte";

  let isBattling: boolean = false
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
    mySquad.setMonsters([])
    for (let i = 0; i < 4; i++) {
      invoke("create_monster", { lvl: 1 }).then((res) => {
        if (isMonster(res)) {
          mySquad.setMonsters([...mySquad.getMonsters(), res])
        }
      })
    }
  })

  $effect(() => {
    enemySquad.newMonsters(1, 4)
  })

  function invokeBattle() {
    invoke("battle", { player: mySquad.getMonsters(), enemy: enemySquad.getMonsters() })
  }
</script>

<main class="container">
  <h1>Incremonsters</h1>

  <button onclick={battleToggle}>Fight!</button>
  {isBattling ? "Battling" : "Not fighting"}
  <section id="battleZone">
    <h3>Power Level: {mySquad.totalStats()}</h3>
    <div class="monsterList">
      {#each mySquad.getMonsters() as monster }
        <div class="monster">
          <h3>{monster.name}</h3>
          <p>Level: {monster.lvl}</p>
          <p>HP: {monster.current_hp}/{monster.hp}</p>
          <p>ATK: {monster.atk}</p>
          <p>DEF: {monster.def}</p>
          <p>SPD: {monster.spd}</p>
        </div>
      {/each}
    </div>
    <h3>Power Level: {enemySquad.totalStats()}</h3>
    <div class="monsterList">
      {#each enemySquad.getMonsters() as monster }
        <div class="monster">
          <h3>{monster.name}</h3>
          <p>Level: {monster.lvl}</p>
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
main {
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: 100%;
  width: 100%;
}
#battleZone {
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
  height: 100%;
}
.monsterList {
  display: flex;
  flex-direction: row;
  justify-content: center;
  text-align: center;
  gap: 30px;
  border-top: 1px solid black;
  width: 50%;
}

p {
  margin: 0;
  padding: 0;
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

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

</style>

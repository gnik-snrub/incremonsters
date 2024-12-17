<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  import { type Monster, isMonster } from "$lib/types/monster"

  /*
  $effect(() => {
    if (enemyMonster.current_hp <= 0) {
      enemyMonster.current_hp = 5
      invoke("create_monster").then((res) => {
        if (isMonster(res)) {
          enemyMonster = res
        }
      })
      playerMonster.exp += enemyMonster.lvl
      if (playerMonster.exp >= playerMonster.lvl * 10) {
        levelUp()
      }
    }
  })

  $effect(() => {
    if (playerMonster.current_hp <= 0) {
      playerMonster.current_hp = playerMonster.hp
    }
  })

  function levelUp() {
    playerMonster.exp -= playerMonster.lvl * 10
    playerMonster.lvl += 1
    playerMonster.atk = Math.round(playerMonster.atk * ((playerMonster.atk + playerMonster.lvl) / playerMonster.atk))
    playerMonster.def = Math.round(playerMonster.def * ((playerMonster.def + playerMonster.lvl) / playerMonster.def))
    playerMonster.spd = Math.round(playerMonster.spd * ((playerMonster.spd + playerMonster.lvl) / playerMonster.spd))
  }

  async function calc() {
    const playerAttack = Math.round(playerMonster.atk * (1 + (playerMonster.spd / 100)))
    const enemyAttack = Math.round(enemyMonster.atk * (1 + (enemyMonster.spd / 100)))
    invoke("calculate_damage", { atk: playerAttack, def: enemyMonster.def })
      .then((res) => {if (typeof res === "number") enemyMonster.current_hp -= res})

    invoke("calculate_damage", { atk: enemyAttack, def: playerMonster.def })
      .then((res) => {if (typeof res === "number") playerMonster.current_hp -= res})
  }
  */

  import { mySquad, enemySquad } from "../stores/monsters.svelte";

  onload = () => {
    let tickspeed = 1000

    setInterval(() => {
      calc()
    }, tickspeed)
  }
  */

  function newMonsters(lvl: number, amount: number) {
    enemySquad.newMonsters(lvl, amount)
  }
</script>

<main class="container">
  <h1>Incremonsters</h1>

  <section id="battleZone">
    <div class="player">
      Quantity: {enemySquad.getMonsters().length}
      Stats: {enemySquad.totalStats()}
    </div>
    <div class="enemy">
      {#each enemySquad.getMonsters() as monster }
        <div class="monster">
          <h3>{monster.name}</h3>
          <p>{monster.lvl}</p>
          <p>{monster.hp}</p>
          <p>{monster.atk}</p>
          <p>{monster.def}</p>
          <p>{monster.spd}</p>
        </div>
      {/each}
      <button onclick={() => newMonsters(200, 4)}>Populate</button>
    </div>
  </section>
</main>

<style>

#battleZone {
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
}
  .enemy {
    display: flex;
    flex-direction: row;
    justify-content: center;
    text-align: center;
    gap: 10px;
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

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  import { type Monster, isMonster } from "$lib/types/monster"

  let enemyMonster: Monster = $state({
    name: "Monster",
    hp: 0,
    current_hp: 0,
    atk: 0,
    def: 0,
    spd: 0,
    exp: 0,
    lvl: 0,
  })

  let playerMonster: Monster = $state({
    name: "Monster",
    hp: 0,
    current_hp: 0,
    atk: 0,
    def: 0,
    spd: 0,
    exp: 0,
    lvl: 0,
  })

  onload = () => {
    invoke("create_monster").then((res) => {
      if (isMonster(res)) {
        enemyMonster = res
      }
    })
  }
  
  function setPlayerMonster() {
    invoke("create_monster").then((res) => {
      if (isMonster(res)) {
        playerMonster = res
      }
    })
  }

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
  
</script>

<main class="container">
  <h1>Incremonsters</h1>

  <section id="battleZone">
    <div class="player">
      <p>Name: {playerMonster.name}</p>
      <p>HP: {playerMonster.current_hp}/{playerMonster.hp}</p>
      <p>ATK: {playerMonster.atk}</p>
      <p>DEF: {playerMonster.def}</p>
      <p>SPD: {playerMonster.spd}</p>
      <p>EXP: {playerMonster.exp}</p>
      <p>LVL: {playerMonster.lvl}</p>
      <button onclick={setPlayerMonster}>New Monster</button>
    </div>
    <div class="enemy">
      <p>Name: {enemyMonster.name}</p>
      <p>HP: {enemyMonster.current_hp}/{enemyMonster.hp}</p>
      <p>ATK: {enemyMonster.atk}</p>
      <p>DEF: {enemyMonster.def}</p>
      <p>SPD: {enemyMonster.spd}</p>
      <p>EXP: {enemyMonster.exp}</p>
      <p>LVL: {enemyMonster.lvl}</p>
    </div>

  <button onclick={calc}>Attack</button>
  </section>
</main>

<style>

#battleZone {
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
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

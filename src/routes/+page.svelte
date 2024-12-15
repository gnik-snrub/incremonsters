<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type Monster = {
    name: String,
    hp: number,
    current_hp: number,
    atk: number,
    def: number,
    spd: number,
    exp: number,
    lvl: number,
  }

  function isMonster(obj: any): obj is Monster {
    return typeof obj === "object" &&
      typeof obj.name === "string" &&
      typeof obj.hp === "number" &&
      typeof obj.current_hp === "number" &&
      typeof obj.atk === "number" &&
      typeof obj.def === "number" &&
      typeof obj.spd === "number" &&
      typeof obj.exp === "number" &&
      typeof obj.lvl === "number"
  }

  let monster: Monster = $state({
    name: "",
  import { type Monster, isMonster } from "$lib/types/monster"
    hp: 0,
    current_hp: 0,
    atk: 0,
    def: 0,
    spd: 0,
    exp: 0,
    lvl: 0,
  })

  async function newMonster() {
    invoke("create_monster")
      .then(new_monster => {
        if (isMonster(new_monster)) {
          monster = new_monster
        }
      })
  }

  let a = $state(0)
  let b = $state(0)
  let out = $state(0)

  async function calc() {
    console.log('atk: ', a, 'def: ', b)
    out = await invoke("calculate_damage", { atk: a, def: b })
    console.log(out, Math.round(a * (a / (a + b))))
  }
</script>

<main class="container">
  <h1>Incremonsters</h1>

  <h3>Monster</h3>
  <p>Name: {monster.name}</p>
  <p>HP: {monster.current_hp}/{monster.hp}</p>
  <p>ATK: {monster.atk}</p>
  <p>DEF: {monster.def}</p>
  <p>SPD: {monster.spd}</p>
  <p>EXP: {monster.exp}</p>
  <p>LVL: {monster.lvl}</p>

  <button onclick={newMonster}>New</button>

  <h3>Calculator</h3>
  <input bind:value={a} type="number" />
  <input bind:value={b} type="number" />
  <button onclick={calc}>Calculate</button>
  <p>{out}</p>
</main>

<style>

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

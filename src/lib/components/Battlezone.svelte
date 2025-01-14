<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  import { type Monster, isMonster } from "$lib/types/monster"
  import { playerSquad, enemySquad, stable } from "../../stores/monsters.svelte";
  import { gold } from "../../stores/resources.svelte";
  import { battle } from "../../stores/battleState.svelte";
    import {onMount} from "svelte";

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

  onMount(() => {
    enemySquad.newMonsters(dungeonLvl, 4)
  })

  let dungeonLvl: number = $state(1)
  let pendingDungeonLevelIncrease: boolean = $state(false)
  let battleAutomation: boolean = false

  async function handleBattleWin(updatedPlayerMonsters: Monster[], goldEarned: number) {
    playerSquad.setMonsters(updatedPlayerMonsters)
    gold.addGold(goldEarned)

    const tamedMonsterIdx: number = Math.random() * (enemySquad.getMonsters().length - 1) | 0
    const tamedMonster: Monster = enemySquad.getMonsters()[tamedMonsterIdx]
    stable.setMonsters([...stable.getMonsters(), tamedMonster])

    pendingDungeonLevelIncrease = true

    // TODO - After battle automation implemented, remove the ! from this
    if (!battleAutomation) {
      await nextDungeon()
    }
  }

  async function nextDungeon() {
    if (pendingDungeonLevelIncrease) {
      dungeonLvl += 1
      await enemySquad.newMonsters(dungeonLvl, 4)
      battle.battleToggle()
      pendingDungeonLevelIncrease = false
    }
  }

  let calculatingRewards: boolean = false

  $effect(async () => {
    if (battle.isBattling() && enemySquad.getAllDead() && !calculatingRewards && !pendingDungeonLevelIncrease) {
      calculatingRewards = true
      battle.battleToggle()

      const response = await invoke("win_battle_rewards", { dungeonLvl, player: playerSquad.getMonsters(), enemy: enemySquad.getMonsters() })
      await handleBattleWin(response[0], response[1])
      calculatingRewards = false
    }
  })

  $effect(() => {
    if (battle.isBattling() && playerSquad.getAllDead()) {
      dungeonLvl = 1
      battle.reset()
    }
  })

</script>

<p style:color="white">Dungeon level: {dungeonLvl}</p>
  <section class="flex flex-col w-full bg-green-500 align-center row-span-2 col-span-6 row-start-3">
    <div class="flex flex-row justify-center w-full gap-10">
      <button onclick={battle.battleToggle}>{battle.isBattling() ? 'Stop fighting' : 'Fight!'}</button>
      <button onclick={battle.reset}>Reset</button>
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
      {#each playerSquad.upgradedMonsters() as monster }
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

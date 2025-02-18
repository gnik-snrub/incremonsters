<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  import { type Monster, isMonster } from "$lib/types/monster"
  import { playerSquad, enemySquad, stable } from "../../stores/monsters.svelte";
  import { gold } from "../../stores/resources.svelte";
  import { battle, dungeonLvl } from "../../stores/battleState.svelte";
  import { rewardBoosts as shopRewardBoosts, intermissionEffects } from '../../stores/shop.svelte'
  import { rewardBoosts as prestigeRewardBoosts, monsterBoosts as prestigeMonsterBoosts } from '../../stores/prestige.svelte'
  import { onMount } from "svelte";

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
    enemySquad.newMonsters(dungeonLvl.get(), 4)
  })

  let pendingDungeonLevelIncrease: boolean = $state(false)
  let battleAutomation: boolean = false

  async function handleBattleWin(updatedPlayerMonsters: Monster[], goldEarned: number) {
    playerSquad.setMonsters(updatedPlayerMonsters)
    gold.add(goldEarned)

    const tamedMonsterIdx: number = Math.random() * (enemySquad.getMonsters().length - 1) | 0
    const tamedMonster: Monster = enemySquad.getMonsters()[tamedMonsterIdx]
    stable.setMonsters([...stable.getMonsters(), tamedMonster])

    pendingDungeonLevelIncrease = true

    for (let i = 0; i < intermissionEffects.length; i++) {
      intermissionEffects[i].run()
    }

    // TODO - After battle automation implemented, remove the ! from this
    if (!battleAutomation) {
      await nextDungeon()
    }
  }

  async function nextDungeon() {
    if (pendingDungeonLevelIncrease) {
      dungeonLvl.increment(1)
      await enemySquad.newMonsters(dungeonLvl.get(), 4)
      battle.battleToggle()
      pendingDungeonLevelIncrease = false
    }
  }

  let calculatingRewards: boolean = false

  $effect(async () => {
    if (battle.isBattling() && enemySquad.getAllDead() && !calculatingRewards && !pendingDungeonLevelIncrease) {
      calculatingRewards = true
      battle.battleToggle()

      let expModifiers = []
      let goldModifiers = []

      for (const boost of [...shopRewardBoosts, ...prestigeRewardBoosts]) {
        if (boost.effectType === 'exp') {
          expModifiers.push(boost.coreEffect())
        } else if (boost.effectType === 'gold') {
          goldModifiers.push(boost.coreEffect())
        }
      }

      const rewardModifiers = {
        exp: expModifiers,
        gold: goldModifiers,
        atk: [prestigeMonsterBoosts[0].coreEffect()],
        def: [prestigeMonsterBoosts[1].coreEffect()],
        spd: [prestigeMonsterBoosts[2].coreEffect()],
        hp: [prestigeMonsterBoosts[3].coreEffect()]
      }
      const response = await invoke("win_battle_rewards", {
        dungeonLvl: dungeonLvl.get(),
        player: playerSquad.getMonsters(),
        enemy: enemySquad.getMonsters(),
        rewardModifiers,
      })
      await handleBattleWin(response[0], response[1])
      calculatingRewards = false
    }
  })

  $effect(() => {
    if (battle.isBattling() && playerSquad.getAllDead()) {
      dungeonLvl.reset()
      battle.reset()
      enemySquad.newMonsters(dungeonLvl.get(), 4)
    }
  })

</script>

  <section class="flex flex-col w-full bg-green-500 align-center row-span-2 col-span-6 row-start-3">
    <div class="flex flex-row justify-center w-full gap-10">
      <button onclick={battle.battleToggle}>{battle.isBattling() ? 'Stop fighting' : 'Fight!'}</button>
    </div>
    <div class="flex flex-row self-center w-3/5 pt-8 border-t-4 border-black justify-evenly gap-4">
      {#each enemySquad.getMonsters() as monster }
        <div class="border-black">
          <h3>{monster.name}</h3>
          <p>Level: {monster.lvl}</p>
          <p>HP: {monster.hp - monster.damage}/{monster.hp}</p>
          <p>ATK: {monster.atk}</p>
          <p>DEF: {monster.def}</p>
          <p>SPD: {monster.spd}</p>
        </div>
      {/each}
    </div>
    <div class="flex flex-row self-center w-3/5 pt-8 mt-8 border-t-4 border-black justify-evenly gap-4">
      {#each playerSquad.upgradedMonsters() as monster, index }
        <div>
          <h3>{monster.name}</h3>
          <p>Level: {monster.lvl}</p>
          <p>EXP: {monster.exp}</p>
          <p>HP: {playerSquad.upgradedMonsters()[index].hp - monster.damage}/{monster.hp}</p>
          <p>ATK: {monster.atk}</p>
          <p>DEF: {monster.def}</p>
          <p>SPD: {monster.spd}</p>
        </div>
      {/each}
    </div>
  </section>

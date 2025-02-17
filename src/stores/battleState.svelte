<script lang="ts" module>
  import { playerSquad, enemySquad } from "./monsters.svelte";
  import { globalModifiers } from "./modifiers.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Monster } from "../lib/types/monster";

  let battleInterval: number = 1000
  let battleIntervalId: ReturnType<typeof setInterval>

  export const dungeonLvl = $state(createDungeonLvl())

  function createDungeonLvl()  {
    let lvl: number = $state(1)
    function get(): number {
      return lvl
    }

    function increment(add: number = 0): void {
      lvl += add
    }

    function reset(): void {
      lvl = 1
    }

    let peakDungeonLvl = 0
    let highestDungeonLvl: number = $derived.by(() => {
      peakDungeonLvl = Math.max(peakDungeonLvl, lvl)
      return peakDungeonLvl
    })

    function getPeak(): number {
      return highestDungeonLvl
    }

    function resetPeak(): void {
      peakDungeonLvl = 1
      reset()
    }
    return { get, increment, reset, getPeak, resetPeak }
  }

  export const battle = battleState()

  function battleState() {
    let battleActive: boolean = $state(false)

    function battleToggle(): void {
      if (battleActive) {
        clearInterval(battleIntervalId)
        battleActive = false
      } else {
        battleIntervalId = setInterval(() => {
          invokeBattle()
        }, battleInterval)
        battleActive = true
      }
    }

    function reset(): void {
      clearInterval(battleIntervalId)
      battleActive = false
      playerSquad.heal(100)
    }

    function invokeBattle(): void {
      invoke("battle", { player: playerSquad.getMonsters(), enemy: enemySquad.getMonsters(), globalModifiers: globalModifiers() })
        .then((res) => {
          const newPlayer: Monster[] = res[0]
          const newEnemy: Monster[] = res[1]
          playerSquad.setMonsters(newPlayer)
          enemySquad.setMonsters(newEnemy)
        })
    }

    function isBattling(): boolean {
      return battleActive
    }

    return { battleToggle, reset, isBattling }
  }
</script>

<script lang="ts" module>
  import { playerSquad, enemySquad } from "./monsters.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Monster } from "../lib/types/monster";

  let battleInterval: number = 1000
  let battleIntervalId: ReturnType<typeof setInterval>

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
      playerSquad.heal()
    }

    function invokeBattle(): void {
      invoke("battle", { player: playerSquad.getMonsters(), enemy: enemySquad.getMonsters() })
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

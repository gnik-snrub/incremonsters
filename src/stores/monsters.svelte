<script lang="ts" module>
  import { invoke } from "@tauri-apps/api/core"

  import { type Monster, isMonster } from "$lib/types/monster"
  import { type MonsterGroup, type EnemyMonsterGroup, type PlayerMonsterGroup } from "$lib/types/monstergroup"
  import { monsterBoosts } from "./shop.svelte"

  export const stable: MonsterGroup = monsterGroup()
  export const playerSquad: PlayerMonsterGroup = playerMonsters()
  export const enemySquad: EnemyMonsterGroup = enemyMonsters()

  function monsterGroup(): MonsterGroup {
    let monsters: Monster[] = $state([])

    let allDead: boolean = $derived.by(() => {
      let total = 0
      for (const monster of monsters) {
        total += monster.current_hp
      }
      let returnValue = total <= 0 ? true : false
      return returnValue
    })

    function getAllDead(): boolean {
      return allDead
    }

    function totalStats(): number {
      let total = 0
      for (let i = 0; i < monsters.length; i++) {
        total += (monsters[i].hp / 5) + monsters[i].atk + monsters[i].def + monsters[i].spd
      }
      return Math.ceil(total)
    }

    function findMonster(id: String): number {
      for (let i = 0; i < monsters.length; i++) {
        if (monsters[i].id === id) {
          return i
        }
      }
      return -1
    }

    function getMonster(id: String): Monster {
      return monsters[findMonster(id)]
    }

    function setMonster(updateMonster: Monster): void {
      const monsterIdx = findMonster(updateMonster.id)
      if (monsterIdx === -1) {
        return
      }
      const newMonsters = [...monsters]
      newMonsters[monsterIdx] = updateMonster
      monsters = [...newMonsters]
    }

    function getMonsters(): Monster[] {
      return monsters
    }

    function setMonsters(newMonsters: Monster[]) {
      monsters = newMonsters
    }

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster }
  }

  export function playerMonsters(): PlayerMonsterGroup {
    let { getMonsters, setMonsters, totalStats, getMonster, setMonster, getAllDead } = monsterGroup()

    function heal(): void {
      const healed = getMonsters().map((monster, index) => {
        return {
          ...monster,
          current_hp: upgradedMonsters()[index].hp
        }
      })
      setMonsters(healed)
    }

    let applyUpgrades = $derived.by(() => {
      const [atkBoost, defBoost, spdBoost, hpBoost] = monsterBoosts
      return getMonsters().map((monster: Monster) => {
        return {
          ...monster,
          atk: Math.floor(monster.atk * ((atkBoost.amountBought() * atkBoost.effectMagnitude) + 1)),
          def: Math.floor(monster.def * ((defBoost.amountBought() * defBoost.effectMagnitude) + 1)),
          spd: Math.floor(monster.spd * ((spdBoost.amountBought() * spdBoost.effectMagnitude) + 1)),
          hp: Math.floor(monster.hp * ((hpBoost.amountBought() * hpBoost.effectMagnitude) + 1)),
        }
      })
    })

    function upgradedMonsters(): Monster[] {
      return applyUpgrades
    }

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster, heal, upgradedMonsters }
  }

  export function enemyMonsters(): EnemyMonsterGroup {

    let { getMonsters, setMonsters, totalStats, getMonster, setMonster, getAllDead } = monsterGroup()

    async function newMonsters(lvl: number, amount: number) {
      setMonsters([])
      const monsterPromises = []
      for (let i = 0; i < amount; i++) {
        let promise = invoke("create_monster", { lvl }).then((res) => {
          if (isMonster(res)) {
            setMonsters([...getMonsters(), res])
          }
        })
        monsterPromises.push(promise)
      }

      await Promise.all(monsterPromises)
    }

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster, newMonsters }
  }
</script>

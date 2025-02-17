<script lang="ts" module>
  import { invoke } from "@tauri-apps/api/core"

  import { type Monster, isMonster } from "$lib/types/monster"
  import { type MonsterGroup, type EnemyMonsterGroup, type PlayerMonsterGroup } from "$lib/types/monstergroup"
  import { globalModifiers } from "./modifiers.svelte"
  import { ModType } from "$lib/types/modifiers";

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

    function reset(): void {
      monsters = []
    }

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster, reset }
  }

  export function playerMonsters(): PlayerMonsterGroup {
    let { getMonsters, setMonsters, totalStats, getMonster, setMonster, getAllDead, reset } = monsterGroup()

    function heal(healRate: number): void {
      const healed = getMonsters().map((monster, index) => {
        const heals = Math.ceil(upgradedMonsters()[index].hp * (healRate / 100))
        return heals + monster.current_hp > upgradedMonsters()[index].hp
          ? {
            ...monster,
            current_hp: upgradedMonsters()[index].hp
          } : {
            ...monster,
            current_hp: monster.current_hp + heals
          }
      })
      setMonsters(healed)
    }

    let applyUpgrades = $derived.by(() => {
      return playerSquad.getMonsters().map((monster: Monster) => {
        let returnMonster = { ...monster }
        let atkMod = 1
        for (let modifier of globalModifiers().atk) {
          switch (modifier.modType) {
            case ModType.ADD:
              atkMod += modifier.modValue
              break
            case ModType.MULT:
              atkMod *= modifier.modValue
              break
            case ModType.SUB:
              atkMod -= modifier.modValue
              break
            case ModType.DIV:
              atkMod /= modifier.modValue
              break
            default:
              break
          }
        }
        returnMonster.atk = Math.round(monster.atk * atkMod)

        let defMod = 1
        for (let modifier of globalModifiers().def) {
          switch (modifier.modType) {
            case ModType.ADD:
              defMod += modifier.modValue
              break
            case ModType.MULT:
              defMod *= modifier.modValue
              break
            case ModType.SUB:
              defMod -= modifier.modValue
              break
            case ModType.DIV:
              defMod /= modifier.modValue
              break
            default:
              break
          }
        }
        returnMonster.def = Math.round(monster.def * defMod)

        let spdMod = 1
        for (let modifier of globalModifiers().spd) {
          switch (modifier.modType) {
            case ModType.ADD:
              spdMod += modifier.modValue
              break
            case ModType.MULT:
              spdMod *= modifier.modValue
              break
            case ModType.SUB:
              spdMod -= modifier.modValue
              break
            case ModType.DIV:
              spdMod /= modifier.modValue
              break
            default:
              break
          }
        }
        returnMonster.spd = Math.round(monster.spd * spdMod)

        let hpMod = 1
        for (let modifier of globalModifiers().hp) {
          switch (modifier.modType) {
            case ModType.ADD:
              hpMod += modifier.modValue
              break
            case ModType.MULT:
              hpMod *= modifier.modValue
              break
            case ModType.SUB:
              hpMod -= modifier.modValue
              break
            case ModType.DIV:
              hpMod /= modifier.modValue
              break
            default:
              break
          }
        }
        returnMonster.hp = Math.round(monster.hp * hpMod)

        return returnMonster
      })
    })

    function upgradedMonsters(): Monster[] {
      return applyUpgrades
    }

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster, heal, upgradedMonsters, reset }
  }

  export function enemyMonsters(): EnemyMonsterGroup {

    let { getMonsters, setMonsters, totalStats, getMonster, setMonster, getAllDead, reset } = monsterGroup()

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

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster, newMonsters, reset }
  }
</script>

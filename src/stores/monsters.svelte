<script lang="ts" module>
  import { invoke } from "@tauri-apps/api/core"

  import { type Monster, isMonster } from "$lib/types/monster"
  import { type MonsterGroup, type EnemyMonsterGroup, type PlayerMonsterGroup } from "$lib/types/monstergroup"
  import { globalModifiers } from "./modifiers.svelte"
  import { ModMode, ModType } from "$lib/types/modifiers";
  import { equipment } from "$lib/entities/shop/equipment.svelte";
  import { EquipmentType, type Equipment } from "$lib/types/shop";

  export const stable: MonsterGroup = monsterGroup()
  export const playerSquad: PlayerMonsterGroup = playerMonsters()
  export const enemySquad: EnemyMonsterGroup = enemyMonsters()

  function monsterGroup(): MonsterGroup {
    let monsters: Monster[] = $state([])

    let allDead: boolean = $derived.by(() => {
      let total = 0
      for (const monster of monsters) {
        total += monster.hp - monster.damage
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
    let { getMonsters, setMonsters, totalStats, getMonster, setMonster, reset } = monsterGroup()

    let equipments: Equipment[] = $state([equipment(0, 'Empty'), equipment(0, 'Empty'), equipment(0, 'Empty'), equipment(0, 'Empty')])
    function setEquipment(equipment: Equipment, index: number): void {
      const newEquipments = [...equipments]
      newEquipments[index] = equipment
      equipments = [...newEquipments]
    }

    function getEquipment(): Equipment[] {
      return equipments
    }

    function heal(healRate: number): void {
      const healed = getMonsters().map((monster, index) => {
        const heals = Math.ceil(upgradedMonsters()[index].hp * (healRate / 100))
        return {
          ...monster,
          damage: monster.damage - heals,
        }
      })
      setMonsters(healed)
    }

    let allDead: boolean = $derived.by(() => {
      let total = 0
      for (const monster of upgradedMonsters()) {
        total += monster.hp - monster.damage
      }
      let returnValue = total <= 0 ? true : false
      return returnValue
    })

    function getAllDead(): boolean {
      return allDead
    }

    function applyModifier(value: number, modifier: ModMode, modValue: number): number {
      switch (modifier) {
        case ModMode.ADD:
          return value + modValue
        case ModMode.MULT:
          return value * modValue
        case ModMode.SUB:
          return value - modValue
        case ModMode.DIV:
          return value / modValue
        default:
          return value
      }
    }

    let applyUpgrades = $derived.by(() => {
      return playerSquad.getMonsters().map((monster: Monster, index: number) => {
        let returnMonster = { ...monster }
        let atkMod = 1
        for (let modifier of globalModifiers().atk) {
          atkMod = applyModifier(atkMod, modifier.modType, modifier.modValue)
        }
        returnMonster.atk = Math.round(monster.atk * atkMod)
        if (equipments[index].type === EquipmentType.WEAPON) {
          returnMonster.atk += equipments[index].value
        }

        let defMod = 1
        for (let modifier of globalModifiers().def) {
          switch (modifier.modType) {
            case ModMode.ADD:
              defMod += modifier.modValue
              break
            case ModMode.MULT:
              defMod *= modifier.modValue
              break
            case ModMode.SUB:
              defMod -= modifier.modValue
              break
            case ModMode.DIV:
              defMod /= modifier.modValue
              break
            default:
              break
          }
        }
        returnMonster.def = Math.round(monster.def * defMod)
        if (equipments[index].type === EquipmentType.ARMOR) {
          returnMonster.def += equipments[index].value
        }

        let spdMod = 1
        for (let modifier of globalModifiers().spd) {
          switch (modifier.modType) {
            case ModMode.ADD:
              spdMod += modifier.modValue
              break
            case ModMode.MULT:
              spdMod *= modifier.modValue
              break
            case ModMode.SUB:
              spdMod -= modifier.modValue
              break
            case ModMode.DIV:
              spdMod /= modifier.modValue
              break
            default:
              break
          }
        }
        returnMonster.spd = Math.round(monster.spd * spdMod)
        if (equipments[index].type === EquipmentType.BOOTS) {
          returnMonster.spd += equipments[index].value
        }

        let hpMod = 1
        for (let modifier of globalModifiers().hp) {
          switch (modifier.modType) {
            case ModMode.ADD:
              hpMod += modifier.modValue
              break
            case ModMode.MULT:
              hpMod *= modifier.modValue
              break
            case ModMode.SUB:
              hpMod -= modifier.modValue
              break
            case ModMode.DIV:
              hpMod /= modifier.modValue
              break
            default:
              break
          }
        }
        returnMonster.hp = Math.round(monster.hp * hpMod)
        if (equipments[index].type === EquipmentType.AMULET) {
          returnMonster.hp += equipments[index].value
        }

        return returnMonster
      })
    })

    function upgradedMonsters(): Monster[] {
      return applyUpgrades
    }

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster, heal, upgradedMonsters, reset, setEquipment, getEquipment }
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

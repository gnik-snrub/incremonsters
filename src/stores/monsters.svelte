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

    function applyModifier(value: number, modifier: ModMode, mod_value: number): number {
      switch (modifier) {
        case ModMode.ADD:
          return value + mod_value
        case ModMode.MULT:
          return value * mod_value
        case ModMode.SUB:
          return value - mod_value
        case ModMode.DIV:
          return value / mod_value
        default:
          return value
      }
    }

    let applyUpgrades = $derived.by(() => {
      return playerSquad.getMonsters().map((monster: Monster, index: number) => {
        let returnMonster = { ...monster }

        // Global Modifiers are applied
        let atkMod = 1
        for (let modifier of globalModifiers().atk) {
          atkMod = applyModifier(atkMod, modifier.mod_mode, modifier.mod_value)
        }
        returnMonster.atk = Math.round(monster.atk * atkMod)
        if (equipments[index].type === EquipmentType.WEAPON) {
          returnMonster.atk += equipments[index].value
        }

        let defMod = 1
        for (let modifier of globalModifiers().def) {
          defMod = applyModifier(defMod, modifier.mod_mode, modifier.mod_value)
        }
        returnMonster.def = Math.round(monster.def * defMod)
        if (equipments[index].type === EquipmentType.ARMOR) {
          returnMonster.def += equipments[index].value
        }

        let spdMod = 1
        for (let modifier of globalModifiers().spd) {
          spdMod = applyModifier(spdMod, modifier.mod_mode, modifier.mod_value)
        }
        returnMonster.spd = Math.round(monster.spd * spdMod)
        if (equipments[index].type === EquipmentType.BOOTS) {
          returnMonster.spd += equipments[index].value
        }

        let hpMod = 1
        for (let modifier of globalModifiers().hp) {
          hpMod = applyModifier(hpMod, modifier.mod_mode, modifier.mod_value)
        }
        returnMonster.hp = Math.round(monster.hp * hpMod)
        if (equipments[index].type === EquipmentType.AMULET) {
          returnMonster.hp += equipments[index].value
        }

        // Temporary Modifiers are applied
        for (let modifier of monster.temporary_modifiers) {
          switch (modifier.mod_type) {
            case ModType.HP:
              returnMonster.hp = applyModifier(returnMonster.hp, modifier.mod_mode, modifier.mod_value * modifier.quantity)
              break
            case ModType.ATK:
              returnMonster.atk = applyModifier(returnMonster.atk, modifier.mod_mode, modifier.mod_value * modifier.quantity)
              break
            case ModType.DEF:
              returnMonster.def = applyModifier(returnMonster.def, modifier.mod_mode, modifier.mod_value * modifier.quantity)
              break
            case ModType.SPD:
              returnMonster.spd = applyModifier(returnMonster.spd, modifier.mod_mode, modifier.mod_value * modifier.quantity)
              break
          }
        }

        return returnMonster
      })
    })

    function upgradedMonsters(): Monster[] {
      return applyUpgrades
    }

    function resetTemporaryModifiers() {
      let resetSquad = getMonsters().map((monster) => {
        return {
          ...monster,
          temporary_modifiers: [],
        }
      })
      setMonsters(resetSquad)
    }

    return { getMonsters, setMonsters, totalStats, getAllDead, getMonster, setMonster, heal, upgradedMonsters, reset, setEquipment, getEquipment, resetTemporaryModifiers }
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

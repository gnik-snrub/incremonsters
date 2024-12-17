<script lang="ts" module>
  import { invoke } from "@tauri-apps/api/core"

  import { type Monster, isMonster } from "$lib/types/monster"
  import { type MonsterGroup, type EnemyMonsterGroup } from "$lib/types/monstergroup"

  export const stable: MonsterGroup = monsterGroup()
  export const mySquad: MonsterGroup = monsterGroup()
  export const enemySquad: EnemyMonsterGroup = enemyMonsters()

  function monsterGroup(): MonsterGroup {
    let monsters: Monster[] = $state([])

    function totalStats(): number {
      let total = 0
      for (let i = 0; i < monsters.length; i++) {
        total += (monsters[i].hp / 5) + monsters[i].atk + monsters[i].def + monsters[i].spd
      }
      return total
    }

    function findMonster(id: string): number {
      for (let i = 0; i < monsters.length; i++) {
        if (monsters[i].id === id) {
          return i
        }
      }
      return -1
    }

    function getMonsters(): Monster[] {
      return monsters
    }

    function setMonsters(newMonsters: Monster[]) {
      monsters = newMonsters
    }

    return { getMonsters, setMonsters, totalStats, findMonster }
  }

  export function enemyMonsters(): EnemyMonsterGroup {

    let { getMonsters, setMonsters, totalStats, findMonster } = monsterGroup()

    function newMonsters(lvl: number, amount: number) {
      setMonsters([])
      const monsterPromises = []
      for (let i = 0; i < amount; i++) {
        let promise = invoke("create_monster", { lvl }).then((res) => {
          if (isMonster(res)) {
            setMonsters([...getMonsters(), res])
            console.log(res)
          }
        })
        monsterPromises.push(promise)
      }

      Promise.all(monsterPromises).then(() => {
        console.log("monsters: ", getMonsters())
      })
    }

    return { getMonsters, setMonsters, totalStats, findMonster, newMonsters }
  }
</script>

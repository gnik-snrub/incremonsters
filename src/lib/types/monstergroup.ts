import { type Monster } from './monster'
import { type Equipment} from './shop'

export type MonsterGroup = {
  getMonsters: () => Monster[],
  setMonsters: (monsters: Monster[]) => void,
  totalStats: () => number,
  getAllDead: () => boolean,
  getMonster: (id: String) => Monster,
  setMonster: (monster: Monster) => void,
  reset: () => void,
}

export type EnemyMonsterGroup = MonsterGroup & {
  newMonsters: (lvl: number, amount: number) => void,
}

export type PlayerMonsterGroup = MonsterGroup & {
  heal: (healRate: number) => void,
  upgradedMonsters: () => Monster[],
  setEquipment: (equipment: Equipment, index: number) => void,
  getEquipment: () => Equipment[],
}

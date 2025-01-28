import { type Monster } from './monster'

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
  upgradedMonsters: () => Monster[]
}

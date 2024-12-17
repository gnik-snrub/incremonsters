import { type Monster } from './monster'

export type MonsterGroup = {
  getMonsters: () => Monster[],
  setMonsters: (monsters: Monster[]) => void,
  totalStats: () => number,
  findMonster: (id: string) => number
}

export type EnemyMonsterGroup = MonsterGroup & {
  newMonsters: (lvl: number, amount: number) => void
}

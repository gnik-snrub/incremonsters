import { type BattleModifier } from './modifiers'

export type Monster = {
  id: String
  name: String,
  hp: number,
  damage: number,
  atk: number,
  def: number,
  spd: number,
  exp: number,
  lvl: number,
  traits: Trait[],
  battleModifiers: BattleModifier[],
}

type Trait = {
  name: String,
  description: String,
}

export function isMonster(obj: any): obj is Monster {
  return typeof obj === 'object' &&
    typeof obj.id === 'string' &&
    typeof obj.name === 'string' &&
    typeof obj.hp === 'number' &&
    typeof obj.damage === 'number' &&
    typeof obj.atk === 'number' &&
    typeof obj.def === 'number' &&
    typeof obj.spd === 'number' &&
    typeof obj.exp === 'number' &&
    typeof obj.lvl === 'number'
}


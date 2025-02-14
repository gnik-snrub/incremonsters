export type GlobalModifier = {
  sourceId: String,
  name: String,
  description: String,
  modType: ModType,
  modValue: number,
}

export type BattleModifier = {
  sourceId: String,
  modTarget: ModTarget,
  modType: ModType,
  modValue: number,
}

export enum ModTarget {
  HP = 'current_hp',
  ATK = 'atk',
  DEF = 'def',
  SPD = 'spd',
}

export enum ModType {
  MULT = 'MULT',
  ADD = 'ADD',
  SUB = 'SUB',
  DIV = 'DIV',
}


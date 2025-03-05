export type GlobalModifier = {
  sourceId: String,
  name: String,
  description: String,
  modType: ModType,
  modValue: number,
}

export type BattleModifier = {
  source: String,
  mod_mode: ModMode,
  mod_type: ModType,
  mod_value: number,
  quantity: number
}

export enum ModType {
  HP,
  ATK,
  DEF,
  SPD,
}

export enum ModMode {
  MULT = 'Mult',
  ADD = 'Add',
  SUB = 'Sub',
  DIV = 'Div',
}


export type GlobalModifier = {
  source_id: String,
  name: String,
  description: String,
  mod_mode: ModMode,
  mod_value: number,
}

export type BattleModifier = {
  source: String,
  mod_mode: ModMode,
  mod_type: ModType,
  mod_value: number,
  quantity: number
}

export enum ModType {
  HP = 'HP',
  ATK = 'ATK',
  DEF = 'DEF',
  SPD = 'SPD',
}

export enum ModMode {
  MULT = 'Mult',
  ADD = 'Add',
  SUB = 'Sub',
  DIV = 'Div',
}


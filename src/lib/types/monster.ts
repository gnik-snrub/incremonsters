export type Monster = {
  id: String
  name: String,
  hp: number,
  current_hp: number,
  atk: number,
  def: number,
  spd: number,
  exp: number,
  lvl: number,
  traits: Trait[],
  temporaryModifiers: TemporaryModifier[],
}

type Trait = {
  name: String,
  description: String,
}

type TemporaryModifier = {
  source: String, // for example, shop item or trait modifier
  modType: ModType,
  modMode: ModMode,
  modValue: number,
}

enum ModMode {
  Additive,
  Multiplicative,
}

enum ModType {
  HP,
  ATK,
  DEF,
  SPD,
}

export function isMonster(obj: any): obj is Monster {
  return typeof obj === 'object' &&
    typeof obj.id === 'string' &&
    typeof obj.name === 'string' &&
    typeof obj.hp === 'number' &&
    typeof obj.current_hp === 'number' &&
    typeof obj.atk === 'number' &&
    typeof obj.def === 'number' &&
    typeof obj.spd === 'number' &&
    typeof obj.exp === 'number' &&
    typeof obj.lvl === 'number'
}


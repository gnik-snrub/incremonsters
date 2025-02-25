export type StoreItem = {
  name: string,
  amountBought: () => number,
  nextCost: () => number,
  buy: () => void,
  reset: () => void,
  description: string,
}

export type BoostEffect = StoreItem & {
  effectType: string,
  effectMagnitude: number,
  coreEffect: () => {
    quantity: number,
    target: string,
    magnitude: number,
    operation: string,
  },
  description: string,
}

export type IntermissionEffect = StoreItem & {
  purchaseLimit: number | null,
  run: () => void | null,
}

export type PrestigeBoostEffect = BoostEffect & {
  mode: string, // Base vs growth
  operation: string, // Add vs mult
}

export type Equipment = {
  name: string,
  level: number,
  type: EquipmentType,
  value: number,
}

export enum EquipmentType {
  AMULET = 'Amulet',
  WEAPON = 'Weapon',
  ARMOR = 'Armor',
  BOOTS = 'Boots',
  EMPTY = 'Empty',
}

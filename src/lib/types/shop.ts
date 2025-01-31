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

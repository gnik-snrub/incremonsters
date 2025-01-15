export type StoreItem = {
  name: string,
  amountBought: () => number,
  nextCost: () => number,
  buy: () => void,
  reset: () => void,
}

export type BoostEffect = StoreItem & {
  effectType: string,
  effectMagnitude: number,
  coreEffect: () => {
    quantity: number,
    type: string,
    magnitude: number
  },
}

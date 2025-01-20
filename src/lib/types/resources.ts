export type Resource = {
  get: () => number,
  add: (amount: number) => void,
  subtract: (amount: number) => void,
}

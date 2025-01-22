export type Resource = {
  get: () => number,
  add: (amount: number) => void,
  subtract: (amount: number) => void,
}

export type ArcaneShards = Resource & {
  acquirePending: () => void,
  getPending: () => number,
}

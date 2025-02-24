export function calculateScaling(level: number, base: number, scaling: number): number {
  if (scaling === 0) return base * level
  return Math.ceil(base * Math.pow(scaling, level))
}

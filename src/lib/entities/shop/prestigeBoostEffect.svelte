<script lang="ts" module>
  import { type PrestigeBoostEffect } from "$lib/types/shop"
  import { boostEffect } from "./boostEffect.svelte"

  export function prestigeBoostEffect(
    itemName: string,
    itemBaseCost: number,
    itemCostScaling: number,
    describe: string,
    effectType: string,
    effectMagnitude: number,
    operation: string,
    mode: string
  ): PrestigeBoostEffect {
    let { name, amountBought, nextCost, buy, reset, description } =
      boostEffect(itemName, itemBaseCost, itemCostScaling, describe, effectType, effectMagnitude)

    function coreEffect(): { quantity: number, target: string, magnitude: number } {
      let quantity = amountBought()
      return { quantity, target: effectType, magnitude: effectMagnitude, operation: 'mult' }
    }

    return { name, amountBought, nextCost , effectType, effectMagnitude, buy, reset, coreEffect, description, operation, mode }
  }
</script>

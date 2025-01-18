<script lang="ts" module>
  import { type BoostEffect } from "$lib/types/shop"
  import { storeItem } from "./storeItem.svelte"

  export function boostEffect(
    itemName: string,
    itemBaseCost: number,
    itemCostScaling: number,
    describe: string,
    effectType: string,
    effectMagnitude: number
  ): BoostEffect {
    let { name, amountBought, nextCost, buy, reset, description } = storeItem(itemName, itemBaseCost, itemCostScaling, describe)

    function coreEffect(): { quantity: number, target: string, magnitude: number } {
      let quantity = amountBought()
      return { quantity, target: effectType, magnitude: effectMagnitude }
    }

    return { name, amountBought, nextCost , effectType, effectMagnitude, buy, reset, coreEffect, description }
  }
</script>

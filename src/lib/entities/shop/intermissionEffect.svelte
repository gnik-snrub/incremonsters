<script lang="ts" module>
  import { type IntermissionEffect } from "$lib/types/shop"
  import { storeItem } from "./storeItem.svelte"

  export function intermissionEffect(itemName: string, itemBaseCost: number, itemCostScaling: number, describe: string,
    purchaseLimit: number, run: () => void): IntermissionEffect {
    let { name, amountBought, nextCost, buy, reset, description } = storeItem(itemName, itemBaseCost, itemCostScaling, describe)

    return { name, amountBought, nextCost , buy, reset, description, purchaseLimit, run }
  }
</script>

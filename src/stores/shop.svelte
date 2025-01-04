<script lang="ts" module>
  import { type StoreItem, type BoostEffect } from "$lib/types/storeitem"

  function storeItem(name: string, baseCost: number, costScaling: number): StoreItem {
    let quantity: number = $state(0)
    let calculatedCost: number = $derived(Math.floor(baseCost * (Math.pow(costScaling, quantity))))

    function amountBought(): number {
      return quantity
    }

    function nextCost(): number {
      return calculatedCost
    }

    function buy() {
      quantity = quantity + 1
    }

    function reset() {
      quantity = 0
    }

    return { name, amountBought, nextCost, buy, reset }
  }

  function boostEffect(itemName: string, itemBaseCost: number, itemCostScaling: number,
    effectType: string, effectMagnitude: number): BoostEffect {
    let { name, amountBought, nextCost, buy, reset } = storeItem(itemName, itemBaseCost, itemCostScaling)

    return { name, amountBought, nextCost , effectType, effectMagnitude, buy, reset }
  }
</script>

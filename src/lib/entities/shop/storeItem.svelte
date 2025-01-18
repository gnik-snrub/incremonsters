<script lang="ts" module>
  import { type StoreItem } from "$lib/types/shop"

  export function storeItem(name: string, baseCost: number, costScaling: number, description: string): StoreItem {
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

    return { name, amountBought, nextCost, buy, reset, description }
  }
  </script>

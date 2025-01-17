<script lang="ts" module>
  import { type StoreItem, type BoostEffect } from "$lib/types/storeitem"

  export const atkBoost: BoostEffect = $state(boostEffect("Atk Boost", 10, 1.25, "Raises player monster attack by 10% (additive)", "atk", 0.1))
  export const defBoost: BoostEffect = $state(boostEffect("Def Boost", 10, 1.25, "Raises player monster defense by 10% (additive)", "def", 0.1))
  export const spdBoost: BoostEffect = $state(boostEffect("Spd Boost", 10, 1.25, "Raises player monster speed by 10% (additive)", "spd", 0.1))
  export const hpBoost: BoostEffect = $state(boostEffect("HP Boost", 10, 1.25, "Raises player monster HP by 10% (additive)", "hp", 0.1))

  export const monsterBoosts: BoostEffect[] = [atkBoost, defBoost, spdBoost, hpBoost]

  export const expBoost: BoostEffect = $state(boostEffect("EXP Boost", 10, 1.25, "Raises player monster EXP by 10% (additive)", "exp", 0.1))
  export const goldBoost: BoostEffect = $state(boostEffect("Gold Boost", 10, 1.25, "Gold gain increase by 10% (additive)", "gold", 0.1))

  export const rewardBoosts: BoostEffect[] = [expBoost, goldBoost]

  function storeItem(name: string, baseCost: number, costScaling: number, description: string): StoreItem {
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

  function boostEffect(itemName: string, itemBaseCost: number, itemCostScaling: number, describe: string,
    effectType: string, effectMagnitude: number): BoostEffect {
    let { name, amountBought, nextCost, buy, reset, description } = storeItem(itemName, itemBaseCost, itemCostScaling, describe)

    function coreEffect(): { quantity: number, type: string, magnitude: number } {
      let quantity = amountBought()
      return { quantity, target: effectType, magnitude: effectMagnitude }
    }

    return { name, amountBought, nextCost , effectType, effectMagnitude, buy, reset, coreEffect, description }
  }
</script>

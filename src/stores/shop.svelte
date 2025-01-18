<script lang="ts" module>
  import { playerSquad } from "./monsters.svelte"

  import { type IntermissionEffect, type BoostEffect } from "$lib/types/shop"
  import { intermissionEffect, boostEffect } from "$entities/shop"

  export const atkBoost: BoostEffect = $state(boostEffect("Atk Boost", 10, 1.25, "Raises player monster attack by 10% (additive)", "atk", 0.1))
  export const defBoost: BoostEffect = $state(boostEffect("Def Boost", 10, 1.25, "Raises player monster defense by 10% (additive)", "def", 0.1))
  export const spdBoost: BoostEffect = $state(boostEffect("Spd Boost", 10, 1.25, "Raises player monster speed by 10% (additive)", "spd", 0.1))
  export const hpBoost: BoostEffect = $state(boostEffect("HP Boost", 10, 1.25, "Raises player monster HP by 10% (additive)", "hp", 0.1))

  export const monsterBoosts: BoostEffect[] = [atkBoost, defBoost, spdBoost, hpBoost]

  export const expBoost: BoostEffect = $state(boostEffect("EXP Boost", 10, 1.25, "Raises player monster EXP by 10% (additive)", "exp", 0.1))
  export const goldBoost: BoostEffect = $state(boostEffect("Gold Boost", 10, 1.25, "Gold gain increase by 10% (additive)", "gold", 0.1))

  export const rewardBoosts: BoostEffect[] = [expBoost, goldBoost]

  export const bandages: IntermissionEffect = $state(intermissionEffect("Bandages", 250, 4, "Heals player monsters by 5%", 5, null))
  bandages.run = function() {
    playerSquad.heal(5 * bandages.amountBought())
  }

  export const intermissionEffects: IntermissionEffect[] = [bandages]
</script>

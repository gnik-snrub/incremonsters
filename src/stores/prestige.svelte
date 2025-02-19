<script lang="ts" module>

  import { type PrestigeBoostEffect } from "$lib/types/shop"
  import { prestigeBoostEffect } from "$entities/shop"

  export const atkBoost: PrestigeBoostEffect = $state(prestigeBoostEffect("Dragon's ember", 10, 1.5, "Raises player monster attack growth by 5% (additive)", "atk", 0.05, 'growth', 'add'))
  export const defBoost: PrestigeBoostEffect = $state(prestigeBoostEffect("Aegis Fragment", 10, 1.5, "Raises player monster defense growth by 5% (additive)", "def", 0.05, 'growth', 'add'))
  export const spdBoost: PrestigeBoostEffect = $state(prestigeBoostEffect("Zephyr Gem", 10, 1.5, "Raises player monster speed growth by 5% (additive)", "spd", 0.05, 'growth', 'add'))
  export const hpBoost: PrestigeBoostEffect = $state(prestigeBoostEffect("Eldertree Seed", 10, 1.5, "Raises player monster HP growth by 5% (additive)", "hp", 0.05, 'growth', 'add'))

  export const monsterBoosts: PrestigeBoostEffect[] = [atkBoost, defBoost, spdBoost, hpBoost]

  export const expBoost: PrestigeBoostEffect = $state(prestigeBoostEffect("Tome of Knowledge", 10, 1.25, "Raises player monster EXP by 5% (multiplicative)", "exp", 0.05, 'mult', 'base'))
  export const goldBoost: PrestigeBoostEffect = $state(prestigeBoostEffect("Alchemist's Coin", 10, 1.25, "Gold gain increase by 5% (multiplicative)", "gold", 0.05, 'mult', 'base'))

  export const rewardBoosts: PrestigeBoostEffect[] = [expBoost, goldBoost]

  export const allShopItems = [atkBoost, defBoost, spdBoost, hpBoost, expBoost, goldBoost]
</script>

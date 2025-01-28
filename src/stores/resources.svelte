<script lang="ts" module>
  import { type Resource, type ArcaneShards } from "$lib/types/resources"
  import { dungeonLvl } from "./battleState.svelte"

  export const gold: Resource = goldFunc()
  export const arcaneShards: ArcaneShards = arcaneShardsFunc()

  function goldFunc(): Resource {
    let gold = $state(0)

    let peakGold = 0
    let highestGold: number = $derived.by(() => {
      peakGold = Math.max(peakGold, gold)
      return peakGold
    })

    function getPeak(): number {
      return highestGold
    }

    function reset(): void {
      peakGold = 0
      gold = 0
    }

    function get(): number {
      return gold
    }

    function add(amount: number): void {
      gold += amount
    }

    function subtract(amount: number): void {
      gold -= amount
    }

    function load(peak: number, current: number): void {
      peakGold = peak
      gold = current
    }

    return { get, add, subtract, getPeak, reset, load }
  }

  function arcaneShardsFunc(): ArcaneShards {
    let shards = $state(0)

    let peakDungeonLvl = 1
    let highestDungeonLvl: number = $derived.by(() => {
      peakDungeonLvl = Math.max(peakDungeonLvl, dungeonLvl.get())
      return peakDungeonLvl
    })

    let peakGold = 0
    let highestGold: number = $derived.by(() => {
      peakGold = Math.max(peakGold, gold.get())
      return peakGold
    })

    let pending = $derived.by(() => {
      return Math.ceil((highestDungeonLvl - 25) * (highestGold / 100000))
    })

    function get(): number {
      return shards
    }

    function add(amount: number) {
      shards += amount
    }

    function subtract(amount: number) {
      shards -= amount
    }

    function acquirePending() {
      add(pending)
    }

    function getPending() {
      return pending
    }

    return { get, add, subtract, acquirePending, getPending }
  }

</script>

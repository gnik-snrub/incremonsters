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

    let peakShards = 0
    let highestShards: number = $derived.by(() => {
      peakShards = Math.max(peakShards, shards)
      return peakShards
    })

    function getPeak(): number {
      return highestShards
    }

    let pending = $derived.by(() => {
      return Math.ceil((dungeonLvl.getPeak() - 25) * (gold.getPeak() / 100000))
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

    function reset() {
      peakShards = 0
      shards = 0
    }

    function load(peak: number, current: number) {
      peakShards = peak
      shards = current
    }

    return { get, add, subtract, acquirePending, getPending, getPeak, reset, load }
  }

</script>

<script lang="ts" module>
  import { type Resource, type ArcaneShards } from "$lib/types/resources"
  import { dungeonLvl } from "./battleState.svelte"

  export const gold: Resource = goldFunc()
  export const arcaneShards: ArcaneShards = arcaneShardsFunc()

  function goldFunc(): Resource {
    let gold = $state(0)

    function get(): number {
      return gold
    }

    function add(amount: number) {
      gold += amount
    }

    function subtract(amount: number) {
      gold -= amount
    }

    return { get, add, subtract }
  }

  function arcaneShardsFunc(): ArcaneShards {
    let shards = $state(0)
    
    let highestDungeonLvl: number = $derived(dungeonLvl.get())
    let highestGold: number = $derived(gold.get())

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

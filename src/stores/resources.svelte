<script lang="ts" module>
  import { type Resource, type ArcaneShards } from "$lib/types/resources"

  export const gold: Resource = goldFunc()

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
</script>

<script lang="ts" module>
  import { type Gold } from "$lib/types/resources"

  export const gold: Gold = goldFunc()

  function goldFunc(): Gold {
    let gold = $state(0)

    function getGold(): number {
      return gold
    }

    function addGold(amount: number) {
      gold += amount
    }

    function subtractGold(amount: number) {
      gold -= amount
    }

    return { getGold, addGold, subtractGold }
  }
</script>

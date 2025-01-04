<script lang="ts">
  import { gold } from "../../stores/resources.svelte"
  import type { StoreItem } from "$lib/types/storeitem"
  import { monsterBoosts } from "../../stores/shop.svelte"

  function buyItem(item: StoreItem) {
    if (gold.getGold() >= item.nextCost()) {
      gold.subtractGold(item.nextCost())
      item.buy()
    }
  }

</script>

<section class="bg-orange-500 col-span-2 row-span-2 grid grid-cols-4 overflow-y-auto grid-rows-[25%] gap-4 p-4">
  {#each monsterBoosts as item}
    <button class="p-1 bg-orange-300 border-2 border-black" onclick={() => {buyItem(item)}}>
      <p>{item.name} ({item.amountBought()})</p>
      <p>Cost: {item.nextCost()}</p>
    </button>
  {/each}
</section>

<script lang="ts">
  import { gold } from "../../stores/resources.svelte"
  import type { StoreItem } from "$lib/types/storeitem"
  import { monsterBoosts, rewardBoosts, intermissionEffects } from "../../stores/shop.svelte"

  const items = [...monsterBoosts, ...rewardBoosts, ...intermissionEffects]

  function buyItem(item: StoreItem) {
    if (gold.get() >= item.nextCost()) {
      gold.subtract(item.nextCost())
      item.buy()
    }
  }

  let itemDescription = $state("")

  function handleEnter(event: Event) {
    itemDescription = (event.currentTarget as HTMLButtonElement).dataset.description || ""
  }

  function handleExit() {
    itemDescription = ""
  }
</script>

<section class="flex flex-col items-center p-4 bg-orange-500 col-span-2 row-span-2">
  <ul class="h-5/6 grid grid-cols-4 overflow-y-auto auto-rows-[25%] w-full gap-4">
    {#each items as item}
      <button class="p-1 bg-orange-300 border-2 border-black max-h-28" data-description={item.description}
        onmouseenter={handleEnter} onmouseleave={handleExit} onclick={() => {buyItem(item)}}>
        <p>{item.name} ({item.amountBought()})</p>
        <p>Cost: {item.nextCost()}</p>
      </button>
    {/each}
  </ul>
  <p class="w-5/6 pt-2 mt-2 text-center border-t-2 border-black">{itemDescription}</p>
</section>

<script lang="ts">
  import { gold } from "../../stores/resources.svelte"
  import type { StoreItem } from "$lib/types/storeitem"
  import { monsterBoosts, rewardBoosts, intermissionEffects } from "../../stores/shop.svelte"

  const boosts = [...monsterBoosts, ...rewardBoosts]
  const campUpgrades = [...intermissionEffects]
  const equipment: StoreItem[] = []

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

  let shopItems: StoreItem[] = $state(boosts)

</script>

<section class="flex flex-col items-center p-4 bg-orange-500 col-span-2 row-span-2">
  <ul class="flex flex-row w-full align-start [&>*]:border-r-2 [&>*]:border-black [&>*]:w-1/4 text-center pb-4">
    <li onclick={() => shopItems = boosts}>Boosts</li>
    <li onclick={() => shopItems = campUpgrades}>Camp Upgrades</li>
    <li onclick={() => shopItems = equipment}>Equipment</li>
  </ul>
    <ul class="h-5/6 grid grid-cols-4 overflow-y-auto auto-rows-[25%] w-full gap-4">
      {#each shopItems as item}
        <button class="p-1 bg-orange-300 border-2 border-black max-h-28" data-description={item.description}
          onmouseenter={handleEnter} onmouseleave={handleExit} onclick={() => {buyItem(item)}}>
          <p>{item.name} ({item.amountBought()})</p>
          <p>Cost: {item.nextCost()}</p>
        </button>
      {/each}
    </ul>
  <p class="h-1.5 w-5/6 pt-2 mt-auto mb-6 text-center border-t-2 border-black">{itemDescription}</p>
</section>

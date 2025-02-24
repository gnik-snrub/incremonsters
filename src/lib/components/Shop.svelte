<script lang="ts">
  import { gold } from "../../stores/resources.svelte"
  import type { StoreItem } from "$lib/types/storeitem"
  import { monsterBoosts, rewardBoosts, intermissionEffects, getPurchasedEquipment, setPurchasedEquipment } from "../../stores/shop.svelte"
  import { dungeonLvl } from "../../stores/battleState.svelte"
  import { calculateScaling } from "$lib/helper/math"
  import { equipment } from "$lib/entities/shop/equipment.svelte"

  const boosts = [...monsterBoosts, ...rewardBoosts]
  const campUpgrades = [...intermissionEffects]

  let equipmentPage: boolean = $state(false)
  let equipmentLvl: number = $state(1)
  let equipmentCost: number = $derived(calculateScaling(equipmentLvl, 1, 1.05))

  function buyEquipment() {
    if (gold.get() >= equipmentCost) {
      gold.subtract(equipmentCost)
      let equipmentType = ''
      switch (Math.floor(Math.random() * 4)) {
        case 0:
          equipmentType = 'Weapon'
          break
        case 1:
          equipmentType = 'Armor'
          break
        case 2:
          equipmentType = 'Boots'
          break
        case 3:
          equipmentType = 'Amulet'
          break
      }
      let newEquipment = [...getPurchasedEquipment(), equipment(equipmentLvl, equipmentType)]
      newEquipment.sort((a, b) => b.level - a.level)
      setPurchasedEquipment(newEquipment)
    }
  }

  function removeEquipment(idx: number) {
    let newEquipment = [...getPurchasedEquipment()]
    newEquipment.splice(idx, 1)
    setPurchasedEquipment(newEquipment)
  }

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
    <li onclick={() => {equipmentPage = false; shopItems = boosts}}>Boosts</li>
    <li onclick={() => {equipmentPage = false; shopItems = campUpgrades}}>Camp Upgrades</li>
    <li onclick={() => equipmentPage = true}>Equipment</li>
  </ul>
  {#if equipmentPage}
  <button onclick={() => equipmentLvl--}>-</button><input type="range" min={1} max={dungeonLvl.getPeak()} bind:value={equipmentLvl}><button onclick={() => equipmentLvl++}>+</button>
    <p>Level: {equipmentLvl}</p>
    <p>Cost: {equipmentCost}</p>
    <button onclick={buyEquipment}>Buy</button>
    <ul class="h-5/6 grid grid-cols-4 overflow-y-auto auto-rows-[25%] w-full gap-4 pb-4">
      {#each getPurchasedEquipment() as item, index}
        <li onclick={() => removeEquipment(index)} class="p-1 bg-orange-300 border-2 border-black max-h-28">
          <p>{item.level} {item.name} {index}</p>
        </li>
      {/each}
    </ul>
  {:else}
    <ul class="h-5/6 grid grid-cols-4 overflow-y-auto auto-rows-[25%] w-full gap-4">
      {#each shopItems as item}
        <button class="p-1 bg-orange-300 border-2 border-black max-h-28" data-description={item.description}
          onmouseenter={handleEnter} onmouseleave={handleExit} onclick={() => {buyItem(item)}}>
          <p>{item.name} ({item.amountBought()})</p>
          <p>Cost: {item.nextCost()}</p>
        </button>
      {/each}
    </ul>
  {/if}
  <p class="h-1.5 w-5/6 pt-2 mt-auto mb-6 text-center border-t-2 border-black">{itemDescription}</p>
</section>

<script lang="ts">
  import { gold } from "../../stores/resources.svelte"
  import { type StoreItem,  type Equipment } from "$lib/types/storeitem"
  import { monsterBoosts, rewardBoosts, intermissionEffects, getPurchasedEquipment, setPurchasedEquipment } from "../../stores/shop.svelte"
  import { playerSquad } from "../../stores/monsters.svelte"
  import { dungeonLvl } from "../../stores/battleState.svelte"
  import { calculateScaling } from "$lib/helper/math"
  import { equipment } from "$lib/entities/shop/equipment.svelte"

  const boosts = [...monsterBoosts, ...rewardBoosts]
  const campUpgrades = [...intermissionEffects]

  let equipmentPage: boolean = $state(false)
  let equipmentLvl: number = $state(1)
  let equipmentCost: number = $derived(calculateScaling(equipmentLvl, 1, 1.1))

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
    const equipmentRemoved = newEquipment.splice(idx, 1)[0]
    playerSquad.getEquipment().forEach((e, i) => {
      if (e.id === equipmentRemoved.id) {
        const empty = equipment(0, 'Empty')
        playerSquad.setEquipment(empty, i)
      }
    })
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

  function handleDragStart(event: DragEvent) {
    event.dataTransfer?.setData('text/plain', event.target?.dataset.idx)
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    const equipmentIdx = event.dataTransfer?.getData('text/plain') || -1
    const positionIdx = event.currentTarget?.dataset.idx || -1

    if (equipmentIdx === -1 || positionIdx === -1) {
      return
    }

    const equipmentCopy = getPurchasedEquipment()[equipmentIdx]
    playerSquad.getEquipment().forEach((e, i) => {
      if (e.id === equipmentCopy.id) {
        playerSquad.setEquipment(equipment(0, 'Empty'), i)
      }
    })
    playerSquad.setEquipment(equipmentCopy, positionIdx)
  }
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
      {#each getPurchasedEquipment() as item, idx}
        <li onclick={() => removeEquipment(idx)}
          ondragstart={handleDragStart}
          data-idx={idx}
          draggable="true"
          class="p-1 bg-orange-300 border-2 border-black max-h-28">
          <p>{item.level} {item.name} {idx}</p>
        </li>
      {/each}
    </ul>
    <ul class="flex flex-row justify-between w-5/6 pt-2 pl-8 pr-8 border-t-2 border-black">
      <li ondrop={handleDrop} ondragover={handleDragOver} data-idx={0}>{playerSquad.getEquipment()[0].name !== 'Empty' ? playerSquad.getEquipment()[0].name : 'First Slot'}</li>
      <li ondrop={handleDrop} ondragover={handleDragOver} data-idx={1}>{playerSquad.getEquipment()[1].name !== 'Empty' ? playerSquad.getEquipment()[1].name : 'Second Slot'}</li>
      <li ondrop={handleDrop} ondragover={handleDragOver} data-idx={2}>{playerSquad.getEquipment()[2].name !== 'Empty' ? playerSquad.getEquipment()[2].name : 'Third Slot'}</li>
      <li ondrop={handleDrop} ondragover={handleDragOver} data-idx={3}>{playerSquad.getEquipment()[3].name !== 'Empty' ? playerSquad.getEquipment()[3].name : 'Fourth Slot'}</li>
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
  <p class="h-1.5 w-5/6 pt-2 mt-auto mb-6 text-center border-t-2 border-black">{itemDescription}</p>
  {/if}
</section>

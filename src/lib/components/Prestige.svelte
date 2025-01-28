<script lang="ts">
  import { gold, arcaneShards } from "../../stores/resources.svelte"
  import { dungeonLvl } from "../../stores/battleState.svelte"
  import { stable, playerSquad, enemySquad } from "../../stores/monsters.svelte"
  import { allShopItems } from "../../stores/shop.svelte"
  import { invoke } from "@tauri-apps/api/core";
  import { isMonster } from "$lib/types/monster";

  let modal: HTMLDialogElement

  function handleOpen() {
    modal.showModal()

    requestAnimationFrame(() => {
      modal.focus({ preventScroll: true })
    })
  }

  function handleClose() {
    modal.close()
  }

  const testItem = {
    name: "testing",
    description: "testing",
    nextCost: () => 10,
    amountBought: () => 0,
    buy: () => {},
  }

  let items = [testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem, testItem]
  let itemDescription = $state("")

  function handleEnter(event: Event) {
    itemDescription = (event.currentTarget as HTMLButtonElement).dataset.description || ""
  }

  function handleExit() {
    itemDescription = ""
  }

  function triggerAwakening() {
    if (arcaneShards.getPending() <= 0) {
      return
    }
    arcaneShards.acquirePending()
    dungeonLvl.resetPeak()
    gold.reset()
    allShopItems.forEach(item => item.reset())
    stable.reset()
    playerSquad.reset()
    enemySquad.reset()


    // Temporary, until more complex monster management implemented
    for (let i = 0; i < 4; i++) {
      invoke("create_monster", { lvl: 1 }).then((res) => {
        if (isMonster(res)) {
          playerSquad.setMonsters([...playerSquad.getMonsters(), res])
        }
      })
    }
    enemySquad.newMonsters(dungeonLvl.get(), 4)
  }
</script>

<section class="bg-yellow-500 col-start-3 row-start-2 grid grid-rows-2 place-items-center">
  <button onclick={triggerAwakening}>Awaken (Pending Shards: {Math.max(arcaneShards.getPending(), 0)})</button>
  <button onclick={handleOpen} class="w-full h-full border-t-2 border-black">Arcane Shards Shop</button>
</section>
<dialog bind:this={modal} class="items-center w-1/2 px-8 bg-yellow-500 border-8 border-black h-1/2 focus:outline-none">
  <div class="flex flex-col items-center w-full h-full p-8">
    <h3>Prestige shop</h3>
    <ul class="h-5/6 grid grid-cols-4 overflow-y-auto auto-rows-[25%] w-full gap-4">
      {#each items as item}
        <button class="p-1 bg-orange-300 border-2 border-black max-h-28" data-description={item.description}
        onmouseenter={handleEnter} onmouseleave={handleExit} onclick={() => {buyItem(item)}}>
          <p>{item.name} ({item.amountBought()})</p>
          <p>Cost: {item.nextCost()}</p>
        </button>
      {/each}
    </ul>
    <p class="w-5/6 h-10 pt-2 mt-2 text-center border-t-2 border-black">{itemDescription}</p>
    <button class="px-2 py-1 border-2 border-black" onclick={handleClose}>Close</button>
  </div>
</dialog>

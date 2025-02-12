<script lang="ts">
    import {type Monster} from "$lib/types/monster";

  import { playerSquad, stable } from "../../stores/monsters.svelte"

  function handleDragStart(event: DragEvent) {
    event.dataTransfer?.setData('text/plain', event.target?.dataset.idx)
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    const stableIdx = event.dataTransfer?.getData('text/plain') || -1
    const squadIdx = event.currentTarget?.dataset.idx || -1

    if (stableIdx === -1 || squadIdx === -1) {
      return
    }

    const tempMonster = playerSquad.getMonsters()[squadIdx]

    playerSquad.setMonsters([
      ...playerSquad.getMonsters().slice(0, squadIdx),
      stable.getMonsters()[stableIdx],
      ...playerSquad.getMonsters().slice(Number(squadIdx) + 1)
    ])
    stable.setMonsters([
      ...stable.getMonsters().slice(0, stableIdx),
      ...stable.getMonsters().slice(Number(stableIdx) + 1),
      tempMonster
    ].sort((a, b) => a.lvl - b.lvl))
  }

  let selectedMonster: Monster | null = $state(null)

  function selectMonster(monster: Monster | null) {
    selectedMonster = monster
  }
</script>

{#if !selectedMonster}
<section class="h-full p-4 bg-red-500 col-span-2 row-span-2 col-start-5 row-start-1 grid grid-rows-[auto_min-content] grid-cols-1">
  <section class="w-full h-full overflow-y-auto grid grid-rows-[1em_auto] gap-4 place-items-center">
    <h4>Stable</h4>
    <div class="h-full w-full overflow-y-auto grid grid-cols-4 gap-4 auto-rows-[25%] place-items-center">
      {#each stable.getMonsters() as monster, idx}
        <div ondragstart={handleDragStart}
          onclick={() => selectMonster(monster)}
          data-idx={idx}
          draggable="true"
          class="w-full h-full border-4 rounded-full border-rose-800 bg-rose-400 grid place-items-center max-w-24 max-h-24">
          <h3 class="select-none">{monster.name}</h3>
        </div>
      {/each}
    </div>
  </section>
  <section class="w-full p-4 border-black border-t-2 grid grid-rows-[1em_auto] gap-4 place-items-center">
    <h4 class="h-min">Squad</h4>
    <div class="w-full grid grid-cols-4 place-items-center">
    {#each playerSquad.getMonsters() as monster, idx}
      <div ondrop={handleDrop} ondragover={handleDragOver} data-idx={idx} class="w-full h-full border-4 rounded-full border-amber-800 bg-amber-400 grid place-items-center max-w-24 max-h-24">
        <h3 class="select-none">{monster.creature_family}</h3>
        <h3 class="select-none">{monster.creature_type}</h3>
      </div>
    {/each}
    </div>
  </section>
</section>
{:else}
<section class="items-center justify-center w-full h-full p-4 bg-red-500 col-span-2 row-span-2 col-start-5 row-start-1 grid grid-cols-2 grid-row-2">
  <div class="w-full h-full bg-amber-400">Image Here</div>
  <section class="pl-4 text-lg mt-1/2">
    <h3 class="underline">Details</h3>
    <h4>{selectedMonster.name}</h4>
    <p>Level: {selectedMonster.lvl}</p>
    <p>HP: {selectedMonster.hp}</p>
    <p>ATK: {selectedMonster.atk}</p>
    <p>DEF: {selectedMonster.def}</p>
    <p>SPD: {selectedMonster.spd}</p>
    <h4>Traits</h4>
    {#each selectedMonster.traits as trait}
      <p>{trait.name}</p>
      <p>{trait.description}</p>
    {/each}
  </section>
  <button class="w-full pt-4 mt-auto border-t-2 border-black col-span-2" onclick={() => selectMonster(null)}>Close</button>
</section>
{/if}

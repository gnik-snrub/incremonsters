<script lang="ts">
  import { gold, arcaneShards } from "../../stores/resources.svelte"
  import { dungeonLvl } from "../../stores/battleState.svelte"
</script>

<section class="flex flex-col bg-blue-500 col-span-2 col-start-3">
  <span>Gold: {gold.get()}</span>
  <span>Arcane Shards: {arcaneShards.get()}</span>
  <span>Deepest Dungeon Level: {dungeonLvl.getPeak()}</span>
  <span>Highest Gold: {gold.getPeak()}</span>
  <button onclick={() => gold.add(1000000000000)}>Add gold</button>
  <button onclick={() => gold.resetPeak()}>Reset Gold</button>
  <button onclick={() => dungeonLvl.resetPeak()}>Reset lvl</button>
  <button onclick={() => dungeonLvl.increment(100)}>Do a big thing</button>
</section>

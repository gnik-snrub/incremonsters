<script lang="ts">
  import { gold, arcaneShards } from "../../stores/resources.svelte"
</script>

<section class="bg-blue-500 col-span-2 col-start-3">
  Gold: {gold.get()}
  Arcane Shards: {arcaneShards.get()}
</section>

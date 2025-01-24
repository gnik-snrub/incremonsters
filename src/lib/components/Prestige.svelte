<script lang="ts">
  let modal: HTMLDialogElement

  function handleOpen() {
    modal.showModal()
  }

  function handleClose() {
    modal.close()
  }

  let items = []
</script>

<section class="bg-yellow-500 col-start-3 row-start-2">
  <h2>Prestige</h2>
  <button onclick={handleOpen}>Open Prestige Shop</button>
</section>
<dialog bind:this={modal} class="items-center w-1/2 px-8 bg-yellow-500 border-8 border-black h-1/2">
  <div class="flex flex-col items-center w-full h-full p-4">
    <h3>Prestige shop</h3>
    <ul class="h-5/6 grid grid-cols-4 overflow-y-auto auto-rows-[25%] w-full gap-4">
      {#each items as item}
        <button class="p-1 bg-orange-300 border-2 border-black max-h-28">
        </button>
      {/each}
    </ul>
    <p class="w-5/6 pt-2 mt-2 text-center border-t-2 border-black">Description goes here</p>
    <button onclick={handleClose} autofocus>Close</button>
  </div>
</dialog>

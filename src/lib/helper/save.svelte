<script lang="ts" module>
  import { playerSquad, stable, enemySquad } from "../../stores/monsters.svelte"
  import { gold } from "../../stores/resources.svelte"
  import { expBoost, goldBoost, atkBoost, defBoost, spdBoost, hpBoost, bandages } from "../../stores/shop.svelte"
  import { battle } from "../../stores/battleState.svelte"
  import { invoke } from "@tauri-apps/api/core"

  export async function save() {
    const saveData = {
      monsterData: {
        player_squad: playerSquad.getMonsters(),
        stable: stable.getMonsters(),
        enemy_squad: enemySquad.getMonsters(),
      },
      resourceData: {
        gold: gold.get(),
      },
      shopData: {
        atk_boost: atkBoost.amountBought(),
        def_boost: defBoost.amountBought(),
        spd_boost: spdBoost.amountBought(),
        hp_boost: hpBoost.amountBought(),

        exp_boost: expBoost.amountBought(),
        gold_boost: goldBoost.amountBought(),

        bandages: bandages.amountBought(),
      },
    }
    await invoke("save", { data: JSON.stringify(saveData) })
  }

  export async function load() {
    if (battle.isBattling()) battle.battleToggle()
    const saveData: string = await invoke("load")
    stable.setMonsters(JSON.parse(saveData).monsterData.stable)
    playerSquad.setMonsters(JSON.parse(saveData).monsterData.player_squad)
    enemySquad.setMonsters(JSON.parse(saveData).monsterData.enemy_squad)

    gold.subtract(gold.get())
    gold.add(JSON.parse(saveData).resourceData.gold)

    atkBoost.reset()
    defBoost.reset()
    spdBoost.reset()
    hpBoost.reset()

    for (let i = 0; i < JSON.parse(saveData).shopData.atk_boost; i++) {
      atkBoost.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).shopData.def_boost; i++) {
      defBoost.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).shopData.spd_boost; i++) {
      spdBoost.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).shopData.hp_boost; i++) {
      hpBoost.buy()
    }

    expBoost.reset()
    goldBoost.reset()

    for (let i = 0; i < JSON.parse(saveData).shopData.exp_boost; i++) {
      expBoost.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).shopData.gold_boost; i++) {
      goldBoost.buy()
    }

    bandages.reset()

    for (let i = 0; i < JSON.parse(saveData).shopData.bandages; i++) {
      bandages.buy()
    }
  }
</script>

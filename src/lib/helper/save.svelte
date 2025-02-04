<script lang="ts" module>
  import { playerSquad, stable, enemySquad } from "../../stores/monsters.svelte"
  import { gold, arcaneShards } from "../../stores/resources.svelte"
  import { expBoost, goldBoost, atkBoost, defBoost, spdBoost, hpBoost, bandages } from "../../stores/shop.svelte"
  import { expBoost as expBoostPrestige,
    goldBoost as goldBoostPrestige,
    atkBoost as atkBoostPrestige,
    defBoost as defBoostPrestige,
    spdBoost as spdBoostPrestige,
    hpBoost as hpBoostPrestige
  } from "../../stores/prestige.svelte"
  import { battle, dungeonLvl } from "../../stores/battleState.svelte"
  import { invoke } from "@tauri-apps/api/core"

  export async function save() {
    const saveData = {
      monsterData: {
        player_squad: playerSquad.getMonsters(),
        stable: stable.getMonsters(),
        enemy_squad: enemySquad.getMonsters(),
      },
      resourceData: {
        gold: {
          current: gold.get(),
          peak: gold.getPeak(),
        },
        arcaneShards: {
          current: arcaneShards.get(),
          peak: arcaneShards.getPeak(),
        }
      },
      battleData: {
        dungeonLvl: {
          current: dungeonLvl.get(),
          peak: dungeonLvl.getPeak(),
        },
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
      arcaneShopData: {
        atk_boost: atkBoostPrestige.amountBought(),
        def_boost: defBoostPrestige.amountBought(),
        spd_boost: spdBoostPrestige.amountBought(),
        hp_boost: hpBoostPrestige.amountBought(),

        exp_boost: expBoostPrestige.amountBought(),
        gold_boost: goldBoostPrestige.amountBought(),
      }
    }
    await invoke("save", { data: JSON.stringify(saveData) })
  }

  export async function load() {
    if (battle.isBattling()) battle.battleToggle()
    const saveData: string = await invoke("load")
    stable.setMonsters(JSON.parse(saveData).monsterData.stable)
    playerSquad.setMonsters(JSON.parse(saveData).monsterData.player_squad)
    enemySquad.setMonsters(JSON.parse(saveData).monsterData.enemy_squad)

    const goldPeak = JSON.parse(saveData).resourceData.gold.peak
    const goldCurrent = JSON.parse(saveData).resourceData.gold.current
    gold.reset()
    gold.load(goldPeak, goldCurrent)

    const shardsPeak = JSON.parse(saveData).resourceData.arcaneShards.peak
    const shardsCurrent = JSON.parse(saveData).resourceData.arcaneShards.current
    arcaneShards.reset()
    arcaneShards.load(shardsPeak, shardsCurrent)

    dungeonLvl.reset()
    dungeonLvl.increment((JSON.parse(saveData).battleData.dungeonLvl.peak - 1))
    dungeonLvl.reset()
    dungeonLvl.increment((JSON.parse(saveData).battleData.dungeonLvl.current - 1))

    // Shop stuff
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

    // Arcane Shards shop stuff

    atkBoostPrestige.reset()
    defBoostPrestige.reset()
    spdBoostPrestige.reset()
    hpBoostPrestige.reset()

    for (let i = 0; i < JSON.parse(saveData).arcaneShopData.atk_boost; i++) {
      atkBoostPrestige.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).arcaneShopData.def_boost; i++) {
      defBoostPrestige.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).arcaneShopData.spd_boost; i++) {
      spdBoostPrestige.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).arcaneShopData.hp_boost; i++) {
      hpBoostPrestige.buy()
    }

    expBoostPrestige.reset()
    goldBoostPrestige.reset()

    for (let i = 0; i < JSON.parse(saveData).arcaneShopData.exp_boost; i++) {
      expBoostPrestige.buy()
    }
    for (let i = 0; i < JSON.parse(saveData).arcaneShopData.gold_boost; i++) {
      goldBoostPrestige.buy()
    }
  }
</script>

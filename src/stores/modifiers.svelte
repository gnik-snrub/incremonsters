<script lang="ts" module>
    import {ModType} from "$lib/types/modifiers";
  import { atkBoost as shopAtkAdd, defBoost as shopDefAdd, spdBoost as shopSpdAdd, hpBoost as shopHpAdd } from "./shop.svelte"

  function getGlobalAtk() {
    return globalAtkModifiers
  }
  const globalAtkModifiers = $derived([
    {
      sourceId: "shopAtkAdd",
      name: shopAtkAdd.name,
      description: shopAtkAdd.description,
      modType: ModType.ADD,
      modValue: shopAtkAdd.amountBought() * shopAtkAdd.effectMagnitude,
    }
  ])
  function getGlobalDef() {
    return globalDefModifiers
  }
  const globalDefModifiers = $derived([
    {
      sourceId: "shopDefAdd",
      name: shopDefAdd.name,
      description: shopDefAdd.description,
      modType: ModType.ADD,
      modValue: shopDefAdd.amountBought() * shopDefAdd.effectMagnitude,
    }
  ])
  function getGlobalSpd() {
    return globalSpdModifiers
  }
  const globalSpdModifiers = $derived([
    {
      sourceId: "shopSpdAdd",
      name: shopSpdAdd.name,
      description: shopSpdAdd.description,
      modType: ModType.ADD,
      modValue: shopSpdAdd.amountBought() * shopSpdAdd.effectMagnitude,
    }
  ])
  function getGlobalHp() {
    return globalHpModifiers
  }
  const globalHpModifiers = $derived([
    {
      sourceId: "shopHpAdd",
      name: shopHpAdd.name,
      description: shopHpAdd.description,
      modType: ModType.ADD,
      modValue: shopHpAdd.amountBought() * shopHpAdd.effectMagnitude,
    }
  ])

  export function globalModifiers() {
    return {
      atk: getGlobalAtk(),
      def: getGlobalDef(),
      spd: getGlobalSpd(),
      hp: getGlobalHp(),
    }
  }
</script>

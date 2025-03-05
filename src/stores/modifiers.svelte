<script lang="ts" module>
    import {ModType} from "$lib/types/modifiers";
  import { atkBoost as shopAtkAdd, defBoost as shopDefAdd, spdBoost as shopSpdAdd, hpBoost as shopHpAdd } from "./shop.svelte"

  function getGlobalAtk() {
    return globalAtkModifiers
  }
  const globalAtkModifiers = $derived([
    {
      source_id: "shopAtkAdd",
      name: shopAtkAdd.name,
      description: shopAtkAdd.description,
      mod_type: ModType.ADD,
      mod_value: shopAtkAdd.amountBought() * shopAtkAdd.effectMagnitude,
    }
  ])
  function getGlobalDef() {
    return globalDefModifiers
  }
  const globalDefModifiers = $derived([
    {
      source_id: "shopDefAdd",
      name: shopDefAdd.name,
      description: shopDefAdd.description,
      mod_type: ModType.ADD,
      mod_value: shopDefAdd.amountBought() * shopDefAdd.effectMagnitude,
    }
  ])
  function getGlobalSpd() {
    return globalSpdModifiers
  }
  const globalSpdModifiers = $derived([
    {
      source_id: "shopSpdAdd",
      name: shopSpdAdd.name,
      description: shopSpdAdd.description,
      mod_type: ModType.ADD,
      mod_value: shopSpdAdd.amountBought() * shopSpdAdd.effectMagnitude,
    }
  ])
  function getGlobalHp() {
    return globalHpModifiers
  }
  const globalHpModifiers = $derived([
    {
      source_id: "shopHpAdd",
      name: shopHpAdd.name,
      description: shopHpAdd.description,
      mod_type: ModType.ADD,
      mod_value: shopHpAdd.amountBought() * shopHpAdd.effectMagnitude,
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

<script lang="ts" module>
  import { EquipmentType, type Equipment } from "$lib/types/shop"
  import { calculateScaling } from "$lib/helper/math"

  export function equipment(
    level: number,
    type: EquipmentType,
  ): Equipment {

    let name = ''
    switch (type) {
      case EquipmentType.AMULET:
        name = "Amulet"
        break
      case EquipmentType.WEAPON:
        name = "Weapon"
        break
      case EquipmentType.ARMOR:
        name = "Armor"
        break
      case EquipmentType.BOOTS:
        name = "Boots"
        break
    }

    const value = calculateScaling(level, 10, 1.05)

    return { name, level, type, value }
  }
</script>

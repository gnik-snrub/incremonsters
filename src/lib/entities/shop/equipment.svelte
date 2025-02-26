<script lang="ts" module>
  import { EquipmentType, type Equipment } from "$lib/types/shop"
  import { calculateScaling } from "$lib/helper/math"
  import { v4 as uuid } from 'uuid'

  export function equipment(
    level: number,
    type: EquipmentType,
  ): Equipment {

    const id = uuid()

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
      case EquipmentType.EMPTY:
        name = "Empty"
        break
    }

    const value = name === 'Empty' ? 0 : calculateScaling(level, 10, 1.05)

    return { id, name, level, type, value }
  }
</script>

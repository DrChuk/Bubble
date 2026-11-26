import { ref } from 'vue'

interface HoverZone {
  id: string
  el: HTMLElement
}

const zones = ref<HoverZone[]>([])
const hoveredId = ref<string | null>(null)

export function useHoverZones() {
  function register(id: string, el: HTMLElement) {
    if (!zones.value.some((z) => z.id === id)) {
      zones.value.push({ id, el })
    }
  }

  function unregister(id: string) {
    zones.value = zones.value.filter((z) => z.id !== id)
    if (hoveredId.value === id) hoveredId.value = null
  }

  function updateHover(cursorX: number, cursorY: number) {
    let found: string | null = null
    for (const zone of zones.value) {
      const rect = zone.el.getBoundingClientRect()
      if (
        cursorX >= rect.left && cursorX <= rect.right &&
        cursorY >= rect.top && cursorY <= rect.bottom
      ) {
        found = zone.id
        break
      }
    }
    if (found !== hoveredId.value) {
      if (hoveredId.value) {
        const prev = zones.value.find((z) => z.id === hoveredId.value)
        prev?.el.classList.remove('hover')
      }
      if (found) {
        const next = zones.value.find((z) => z.id === found)
        next?.el.classList.add('hover')
      }
      hoveredId.value = found
    }
  }

  return { zones, hoveredId, register, unregister, updateHover }
}

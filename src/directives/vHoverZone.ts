import type { Directive } from 'vue'
import { useHoverZones } from '../composables/useHoverZones'

const { register, unregister } = useHoverZones()

export const vHoverZone: Directive<HTMLElement, string> = {
  mounted(el, binding) {
    const id = binding.value || `zone-${Math.random().toString(36).slice(2)}`
    el.dataset.hoverId = id
    register(id, el)
  },
  unmounted(el) {
    const id = el.dataset.hoverId
    if (id) unregister(id)
  },
}

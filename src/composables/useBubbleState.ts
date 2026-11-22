import { ref } from "vue";

export type BubbleState = 'hidden' | 'minimized' | 'expanded';

const state = ref<BubbleState>('hidden');
function setState(next: BubbleState) {
    state.value = next;
}

function hide() { setState('hidden') }
function minimize() { setState('minimized') }
function expand() { setState('expanded') }

export function useBubbleState() {
    return { state, setState, hide, minimize, expand }
}

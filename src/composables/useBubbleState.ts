import { ref } from "vue";

export type BubbleState = 'hidden' | 'minimized' | 'expanded';

export function useBubbleState() {
    const state = ref<BubbleState>('hidden');

    function setState(next: BubbleState) {
        state.value = next;
    }

    function hide() { setState('hidden') }
    function minimize() { setState('minimized') }
    function expand() { setState('expanded') }
    
    return { state, setState, hide, minimize, expand }
}
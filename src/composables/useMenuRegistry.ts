import { ref, computed } from 'vue';
import { APPS } from './../components/appdock/index.ts';

export type AppId = string;

const activeId = ref<AppId>(APPS[0]?.id ?? '');

const activeApp = computed(() => 
    APPS.find((a) => a.id === activeId.value) ?? null
);

function setActive(id: AppId) {
    if (APPS.some((a) => a.id === id)) {
        activeId.value = id;
    }
}

export function useMenuRegistry() {
    return {apps: APPS, activeId, activeApp, setActive};
}

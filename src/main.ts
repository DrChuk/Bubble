import { createApp } from "vue";
import App from "./App.vue";

document.addEventListener('contextmenu', (e) => {
    e.preventDefault();
});

const deniedKeys = ['KeyF', 'KeyA', 'KeyR', 'KeyI', 'KeyP', 'KeyG', 'KeyJ', 'KeyC']

document.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && deniedKeys.includes(e.code)) {
        e.preventDefault();
    }

    if ((e.ctrlKey || e.metaKey) && e.shiftKey && deniedKeys.includes(e.code)) {
        e.preventDefault();
    }

    if (e.key === 'F5' || e.key === 'F12') {
        if (!import.meta.env.DEV) {
            e.preventDefault();
        }
    }
});

createApp(App).mount("#app");

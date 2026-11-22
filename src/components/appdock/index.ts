import Timer from './apps/timer/Main.vue';
import Autokey from './apps/autokey/Main.vue';

export const APPS = [
    {
        id: "timer",
        title: "Timer",
        icon: "/dock-icons/timer.svg",
        component: Timer,
    },
    {
        id: "autokey",
        title: "Autokey",
        icon: "/dock-icons/keyboard.svg",
        component: Autokey,
    }
]

import { onUnmounted, ref } from "vue";

export function useTimer(initialSeconds: number) {
    const total = ref(initialSeconds);
    const remaining = ref(initialSeconds);
    const isRunning = ref(false);
    let intervalId: number | null = null;
    let endTime = 0;

    function tick() {
        const now = Date.now();
        const left = Math.max(0, Math.ceil((endTime - now) / 1000));
        remaining.value = left;
        if (left <= 0) stop();
    }

    function start() {
        if (isRunning.value) return;
        isRunning.value = true;
        endTime = Date.now() + remaining.value * 1000;
        intervalId = setInterval(tick, 250);
    }

    function stop() {
        isRunning.value = false;
        if (intervalId !== null) {
            clearInterval(intervalId);
            intervalId = null;
        }
    }

    function reset() {
        stop()
        remaining.value = total.value;
    }

    function returnString() {
        const minutes = Math.floor(remaining.value / 60);
        const seconds = remaining.value % 60;
        const minutesStr = minutes < 10 ? "0" + minutes : minutes;
        const secondsStr = seconds < 10 ? "0" + seconds : seconds;

        return `${minutesStr}:${secondsStr}`;
    }

    onUnmounted(() => stop());

    return {start, stop, reset, returnString};
}

import { ref } from "vue";
import { DEFAULT_CONFIG } from '../types/config';
import type { AppConfig } from "../types/config";
import { invoke } from "@tauri-apps/api/core";

const config = ref<AppConfig>({ ...DEFAULT_CONFIG});

export function useConfig() {
    async function loadConfig() {
        try {
            config.value = await invoke<AppConfig>('load_config');
        } catch (e) {
            console.error('Failed to load the config: ', e);
            config.value = { ...DEFAULT_CONFIG };
        }
    }

    async function saveConfig() {
        try {
            await invoke('save_config', { config: config.value});
        } catch (e) {
            console.error('Failed to save the config: ', e);
        }
    }

    return {config, loadConfig, saveConfig}
}

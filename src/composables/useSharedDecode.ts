import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const result = ref(null);
const error  = ref<string | null>(null);

export function useSharedDecode() {
    async function run(frame: string) {
        try {
            result.value = await invoke("decode_frame", { frame });
            error.value  = null;
        } catch (e: any) {
            result.value = null;
            error.value  = String(e);
        }
    }
    return { run, result, error };
}
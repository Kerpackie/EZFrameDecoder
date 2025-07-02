import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function useDecode() {
    const decoded = ref<any | null>(null);
    const error   = ref<string | null>(null);

    async function run(frame: string) {
        decoded.value = null;
        error.value = null;
        try {
            decoded.value = await invoke("decode_frame", { frame });
        } catch (e: any) {
            error.value = String(e);
        }
    }
    return { decoded, error, run };
}

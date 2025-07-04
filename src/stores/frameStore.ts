import { defineStore } from "pinia";

export const useFrameStore = defineStore("frames", {
    state: () => ({
        frames: [] as string[],
        selected: -1
    }),
    actions: {
        setFrames(list: string[]) {
            this.frames = list;
            this.selected = -1;
        },
        select(index: number) {
            this.selected = index;
        },
        clear() {
            this.frames = [];
            this.selected = -1;
        }
    }
});

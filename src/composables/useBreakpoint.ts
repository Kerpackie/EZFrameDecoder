import { ref, onMounted, onUnmounted } from "vue";

/**
 * Reactive flag that is true when the media query matches.
 * Defaults to "(max-width: 1200px)".
 */
export function useBreakpoint(query = "(max-width: 1200px)") {
    const match = ref(false);
    let mq!: MediaQueryList;

    const update = () => (match.value = mq.matches);

    onMounted(() => {
        mq = window.matchMedia(query);
        update();
        mq.addEventListener("change", update);
    });

    onUnmounted(() => {
        mq?.removeEventListener("change", update);
    });

    return match;
}

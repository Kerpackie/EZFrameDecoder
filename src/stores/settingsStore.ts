import { ref, watch } from 'vue';

// --- Advanced Mode State ---
const isAdvancedMode = ref(false);
const savedMode = localStorage.getItem('advancedMode');
if (savedMode) {
    isAdvancedMode.value = JSON.parse(savedMode);
}
watch(isAdvancedMode, (newValue) => {
    localStorage.setItem('advancedMode', JSON.stringify(newValue));
});

// --- Dark Mode ---
const isDarkMode = ref(false);
const savedTheme = localStorage.getItem('isDarkMode');
if (savedTheme) {
    isDarkMode.value = JSON.parse(savedTheme);
}
watch(isDarkMode, (newValue) => {
    localStorage.setItem('isDarkMode', JSON.stringify(newValue));
});


// --- Composable ---
export function useSettingsStore() {
    const toggleAdvancedMode = () => {
        isAdvancedMode.value = !isAdvancedMode.value;
    };

    const toggleDarkMode = () => {
        isDarkMode.value = !isDarkMode.value;
    };

    return {
        isAdvancedMode,
        toggleAdvancedMode,
        isDarkMode,
        toggleDarkMode,
    };
}

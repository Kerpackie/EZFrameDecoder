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

// --- Spec File Path State (now for display/source path) ---
const specFilePath = ref<string | null>(null);
const savedSpecPath = localStorage.getItem('specFilePath');
if (savedSpecPath) {
    specFilePath.value = savedSpecPath;
}
watch(specFilePath, (newValue) => {
    if (newValue) {
        localStorage.setItem('specFilePath', newValue);
    } else {
        localStorage.removeItem('specFilePath'); // Remove if null/default
    }
});


// --- Composable ---
export function useSettingsStore() {
    const toggleAdvancedMode = () => {
        isAdvancedMode.value = !isAdvancedMode.value;
    };

    const toggleDarkMode = () => {
        isDarkMode.value = !isDarkMode.value;
    };

    const setSpecFilePath = (path: string | null) => {
        specFilePath.value = path;
    };

    return {
        isAdvancedMode,
        toggleAdvancedMode,
        isDarkMode,
        toggleDarkMode,
        specFilePath,
        setSpecFilePath,
    };
}

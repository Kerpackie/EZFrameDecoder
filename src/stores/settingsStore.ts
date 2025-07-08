import { ref, watch } from 'vue';

// Create a reactive state for advanced mode
const isAdvancedMode = ref(false);

// Load the initial value from localStorage
const savedMode = localStorage.getItem('advancedMode');
if (savedMode) {
    isAdvancedMode.value = JSON.parse(savedMode);
}

// Watch for changes and save to localStorage
watch(isAdvancedMode, (newValue) => {
    localStorage.setItem('advancedMode', JSON.stringify(newValue));
});

// Export a composable function to use the store
export function useSettingsStore() {
    const toggleAdvancedMode = () => {
        isAdvancedMode.value = !isAdvancedMode.value;
    };

    return {
        isAdvancedMode,
        toggleAdvancedMode,
    };
}

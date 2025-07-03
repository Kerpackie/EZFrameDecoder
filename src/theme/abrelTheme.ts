import type { GlobalThemeOverrides } from "naive-ui";

export const abrelTheme: GlobalThemeOverrides = {
    common: {
        // Primary brand red (used in buttons and accents)
        primaryColor: "#e02020",
        primaryColorHover: "#c61c1c",
        primaryColorPressed: "#a31717",
        primaryColorSuppl: "#c61c1c",

        // Accent navy background
        bodyColor: "#f5f7fa",
        cardColor: "#ffffff",
        textColorBase: "#1f1f1f",

        // Borders
        borderColor: "#e0e0e0",

        // Status colors (aligned to brand palette)
        successColor: "#1cb56f",
        warningColor: "#f5a623",
        errorColor: "#d0021b",
        infoColor: "#2962ff"
    },

    Button: {
        textColor: "#ffffff",
        colorHover: "#c61c1c",
        colorPressed: "#a31717",
        borderHover: "#e02020"
    },

    Input: {
        borderColor: "#cfd8dc",
        borderHover: "#e02020",
        placeholderColor: "#9e9e9e"
    },

    Layout: {
        siderColor: "#f1f3f5",
        headerColor: "#ffffff"
    },

    Table: {
        borderColor: "#e0e0e0",
        thColor: "#f5f5f5",
        tdColor: "#ffffff"
    },

    Tooltip: {
        color: "#1f1f1f",
        textColor: "#ffffff"
    },

    Message: {
        color: "#ffffff",
        textColor: "#1f1f1f"
    }
};

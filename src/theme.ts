import type { GlobalThemeOverrides } from "naive-ui";

export const themeOverrides: GlobalThemeOverrides = {
    common: {
        primaryColor:          "#0057B7",
        primaryColorHover:     "#0A64C8",
        primaryColorPressed:   "#004FA6",
        primaryColorSuppl:     "#0A64C8",

        successColor:          "#28C07B",
        warningColor:          "#FFB300",
        errorColor:            "#E03B44",
        infoColor:             "#5CACF4",

        // optional greys / backgrounds
        bodyColor:             "#f9fafb",
        cardColor:             "#ffffff",
    },
    Button: {
        textColor: "#ffffff"
    },
    Input: {
        borderHover: "#0057B7"
    }
};

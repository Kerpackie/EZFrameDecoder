import type { GlobalThemeOverrides } from 'naive-ui'

export const themeOverrides: GlobalThemeOverrides = {
    common: {
        primaryColor:          '#0057B7',
        primaryColorHover:     '#0A64C8',
        primaryColorPressed:   '#004FA6',
        primaryColorSuppl:     '#0A64C8',

        successColor:          '#28C07B',
        warningColor:          '#FFB300',
        errorColor:            '#E03B44',
        infoColor:             '#5CACF4',

        bodyColor:             '#f9fafb',
        cardColor:             '#ffffff'
    },

    Button: {
        /* PRIMARY */
        colorPrimary:           '#0057B7',
        colorHoverPrimary:      '#0A64C8',
        colorPressedPrimary:    '#004FA6',
        textColorPrimary:       '#ffffff',
        borderPrimary:          '1px solid #0057B7',

        /* DEFAULT */
        color:                  '#ffffff',
        colorHover:             '#f5faff',
        colorPressed:           '#e6f2ff',
        textColor:              '#0057B7',
        border:                 '1px solid #B7CCE6',
        borderHover:            '1px solid #89B1E3',
        borderPressed:          '1px solid #6A97D5',

        waveOpacity:            0.2
    },

    Input: {
        color:                  '#ffffff',
        colorFocus:             '#ffffff',
        textColor:              '#000000',

        border:                 '1px solid #B7CCE6',
        borderHover:            '1px solid #89B1E3',
        borderFocus:            '1px solid #0057B7',

        placeholderColor:       '#94A3B8',
        caretColor:             '#0057B7'
    }
}

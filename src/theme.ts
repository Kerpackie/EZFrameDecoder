import { type GlobalThemeOverrides, darkTheme } from 'naive-ui'

const lightThemeOverrides: GlobalThemeOverrides = {
    common: {
        primaryColor:          '#0057B7',
        primaryColorHover:     '#0A64C8',
        primaryColorPressed:   '#004FA6',
        primaryColorSuppl:     '#0A64C8',

        successColor:          '#28C07B',
        warningColor:          '#FFB300',
        errorColor:            '#E03B44',
        infoColor:             '#5CACF4',

        bodyColor:             '#f8f9fa',
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
    },
    Menu: {
        itemBorderRadius: '6px',
    }
};

const darkThemeOverrides: GlobalThemeOverrides = {
    common: {
        primaryColor:          '#3b82f6',
        primaryColorHover:     '#60a5fa',
        primaryColorPressed:   '#2563eb',
        primaryColorSuppl:     '#60a5fa',

        successColor:          '#4ade80',
        warningColor:          '#facc15',
        errorColor:            '#f87171',
        infoColor:             '#60a5fa',

        bodyColor:             '#111827',
        cardColor:             '#1f2937',
        borderColor:           'rgba(255, 255, 255, 0.2)',
        textColorBase:         '#f9fafb',
        textColor1:            '#f3f4f6',
        textColor2:            '#d1d5db',
        textColor3:            '#9ca3af',
    },
    Button: {
        // You can add specific dark mode button overrides here if needed
    },
    Input: {
        color:                  '#374151',
        colorFocus:             '#374151',
        border:                 '1px solid #4b5563',
        borderHover:            '1px solid #6b7280',
        borderFocus:            '1px solid #3b82f6',
        placeholderColor:       '#9ca3af',
        textColor:              '#f9fafb',
    },
    Layout: {
        headerColor: '#1f2937',
        siderColor: '#1f2937',
        siderBorderColor: '#2d3748', // Correctly theme the border
        headerBorderColor: '#2d3748',
    },
    Menu: {
        color: '#1f2937', // Set the background color of the menu
        itemTextColor: '#d1d5db',
        itemTextColorHover: '#ffffff',
        itemColorActive: 'rgba(59, 130, 246, 0.2)',
        itemIconColor: '#9ca3af',
        itemBorderRadius: '6px',
    },
    Card: {
        borderColor: '#4b5563'
    },
    Tabs: {
        tabTextColor: '#9ca3af',
        tabTextColorActive: '#3b82f6',
        barColor: '#3b82f6',
    },
    Collapse: {
        borderColor: '#4b5563'
    },
    Upload: {
        draggerColor: '#374151',
        draggerBorder: '1px dashed #4b5563',
        draggerBorderHover: '1px dashed #6b7280',
    },
    Table: {
        tdColor: '#1f2937',
        thColor: '#374151',
        borderColor: 'rgba(255, 255, 255, 0.12)',
    },
    Descriptions: {
        textColor: '#d1d5db',
        labelTextColor: '#f9fafb',
    },
    Code: {
        textColor: '#e5e7eb',
        color: '#374151',
    },
    Tag: {
        color: '#374151',
        textColor: '#d1d5db',
        borderColor: '#4b5563'
    },
    Empty: {
        textColor: '#9ca3af',
    },
    List: {
        color: '#1f2937',
        textColor: '#d1d5db',
        colorHover: 'rgba(255, 255, 255, 0.04)',
    },
    InternalSelection: { // Used by Select component
        color: '#374151',
        textColor: '#f9fafb',
        border: '1px solid #4b5563',
        borderHover: '1px solid #6b7280',
        borderFocus: '1px solid #3b82f6',
    },
};


export { lightThemeOverrides, darkThemeOverrides, darkTheme };

/** @type {import('tailwindcss').Config} */

const { createThemes } = require('tw-colors');

module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    createThemes({
      dark: { 
        "primary": "#D0BCFE",
        "surfaceTint": "#D0BCFF",
        "onPrimary": "#381E72",
        "primaryContainer": "#4F378B",
        "onPrimaryContainer": "#EADDFF",
        "secondary": "#CCC2DC",
        "onSecondary": "#332D41",
        "secondaryContainer": "#4A4458",
        "onSecondaryContainer": "#E8DEF8",
        "tertiary": "#EFB8C8",
        "onTertiary": "#492532",
        "tertiaryContainer": "#633B48",
        "onTertiaryContainer": "#FFD8E4",
        "error": "#F2B8B5",
        "onError": "#601410",
        "errorContainer": "#8C1D18",
        "onErrorContainer": "#F9DEDC",
        "background": "#141218",
        "onBackground": "#E6E0E9",
        "surface": "#141218",
        "onSurface": "#E6E0E9",
        "surfaceVariant": "#49454F",
        "onSurfaceVariant": "#CAC4D0",
        "outline": "#938F99",
        "outlineVariant": "#49454F",
        "shadow": "#000000",
        "scrim": "#000000",
        "inverseSurface": "#E6E0E9",
        "inverseOnSurface": "#322F35",
        "inversePrimary": "#6750A4",
        "primaryFixed": "#EADDFF",
        "onPrimaryFixed": "#21005D",
        "primaryFixedDim": "#D0BCFF",
        "onPrimaryFixedVariant": "#4F378B",
        "secondaryFixed": "#E8DEF8",
        "onSecondaryFixed": "#1D192B",
        "secondaryFixedDim": "#CCC2DC",
        "onSecondaryFixedVariant": "#4A4458",
        "tertiaryFixed": "#FFD8E4",
        "onTertiaryFixed": "#31111D",
        "tertiaryFixedDim": "#EFB8C8",
        "onTertiaryFixedVariant": "#633B48",
        "surfaceDim": "#141218",
        "surfaceBright": "#3B383E",
        "surfaceContainerLowest": "#0F0D13",
        "surfaceContainerLow": "#1D1B20",
        "surfaceContainer": "#211F26",
        "surfaceContainerHigh": "#2B2930",
        "surfaceContainerHighest": "#36343B"
      }
    }, {
      strict: true
    })
  ],
}


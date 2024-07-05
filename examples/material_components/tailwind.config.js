/** @type {import('tailwindcss').Config} */

const { addIconSelectors } = require("@iconify/tailwind");

module.exports = {
  content: {
    files: [
      "*.html",
      "./src/**/*.rs",
      "../../src/components/cmp_leptos/components/**/*.rs",
    ],
  },
  plugins: [
    require("@tailwindcss/forms"),
    addIconSelectors(["mdi", "material-symbols"]),
  ],
  theme: {
    extend: {
      colors: {
        primary: "var(--md-sys-color-primary)",
        "surface-tint": "var(--md-sys-color-surface-tint)",
        "on-primary": "var(--md-sys-color-on-primary)",
        "primary-container": "var(--md-sys-color-primary-container)",
        "on-primary-container": "var(--md-sys-color-on-primary-container)",
        secondary: "var(--md-sys-color-secondary)",
        "on-secondary": "var(--md-sys-color-on-secondary)",
        "secondary-container": "var(--md-sys-color-secondary-container)",
        "on-secondary-container": "var(--md-sys-color-on-secondary-container)",
        tertiary: "var(--md-sys-color-tertiary)",
        "on-tertiary": "var(--md-sys-color-on-tertiary)",
        "tertiary-container": "var(--md-sys-color-tertiary-container)",
        "on-tertiary-container": "var(--md-sys-color-on-tertiary-container)",
        error: "var(--md-sys-color-error)",
        "on-error": "var(--md-sys-color-on-error)",
        "error-container": "var(--md-sys-color-error-container)",
        "on-error-container": "var(--md-sys-color-on-error-container)",
        background: "var(--md-sys-color-background)",
        "on-background": "var(--md-sys-color-on-background)",
        surface: "var(--md-sys-color-surface)",
        "on-surface": "var(--md-sys-color-on-surface)",
        "surface-variant": "var(--md-sys-color-surface-variant)",
        "on-surface-variant": "var(--md-sys-color-on-surface-variant)",
        outline: "var(--md-sys-color-outline)",
        "outline-variant": "var(--md-sys-color-outline-variant)",
        shadow: "var(--md-sys-color-shadow)",
        scrim: "var(--md-sys-color-scrim)",
        "inverse-surface": "var(--md-sys-color-inverse-surface)",
        "inverse-on-surface": "var(--md-sys-color-inverse-on-surface)",
        "inverse-primary": "var(--md-sys-color-inverse-primary)",
        "primary-fixed": "var(--md-sys-color-primary-fixed)",
        "on-primary-fixed": "var(--md-sys-color-on-primary-fixed)",
        "primary-fixed-dim": "var(--md-sys-color-primary-fixed-dim)",
        "on-primary-fixed-variant":
          "var(--md-sys-color-on-primary-fixed-variant)",
        "secondary-fixed": "var(--md-sys-color-secondary-fixed)",
        "on-secondary-fixed": "var(--md-sys-color-on-secondary-fixed)",
        "secondary-fixed-dim": "var(--md-sys-color-secondary-fixed-dim)",
        "on-secondary-fixed-variant":
          "var(--md-sys-color-on-secondary-fixed-variant) ",
        "tertiary-fixed": "var(--md-sys-color-tertiary-fixed)",
        "on-tertiary-fixed": "var(--md-sys-color-on-tertiary-fixed)",
        "tertiary-fixed-dim": "var(--md-sys-color-tertiary-fixed-dim)",
        "on-tertiary-fixed-variant":
          "var(--md-sys-color-on-tertiary-fixed-variant)",
        "surface-dim": "var(--md-sys-color-surface-dim)",
        "surface-bright": "var(--md-sys-color-surface-bright)",
        "surface-container-lowest":
          "var(--md-sys-color-surface-container-lowest)",
        "surface-container-low": "var(--md-sys-color-surface-container-low)",
        "surface-container": "var(--md-sys-color-surface-container)",
        "surface-container-high": "var(--md-sys-color-surface-container-high)",
        "surface-container-highest":
          "var(--md-sys-color-surface-container-highest)",
        "green-color": "var(--md-extended-color-green-color)",
        "green-on-color": "var(--md-extended-color-green-on-color)",
        "green-color-container":
          "var(--md-extended-color-green-color-container)",
        "green-on-color-container":
          "var (--md-extended-color-green-on-color-container)",
        "yellow-color": "var(--md-extended-color-yellow-color)",
        "yellow-on-color": "var(--md-extended-color-yellow-on-color)",
        "yellow-color-container":
          "var(--md-extended-color-yellow-color-container)",
        "yellow-on-color-container":
          "var(--md-extended-color-yellow-on-color-container)",
      },
    },
  },
};

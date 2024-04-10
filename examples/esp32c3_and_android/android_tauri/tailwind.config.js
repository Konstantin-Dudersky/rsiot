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
        'background': '#121212',
        'surface': '#121212',
        'primary': '#BB86FC',
        'primary_variant': '#3700B3',
        'secondary': '#03DAC6',
        'error': '#CF6679',
        'on_background': '#FFFFFF',
        'on_surface': '#FFFFFF',
        'on_primary': '#000000',
        'on_secondary': '#000000',
        'on_error': '#000000',
      }
    }, {
      strict: true
    })
  ],
}


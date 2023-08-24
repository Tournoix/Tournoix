/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {
      fontFamily: {
        bebas: ['Bebas Neue', 'sans-serif'],
      },

      colors: {
        transparent: 'transparent',
        current: 'currentColor',
        highlight: '#ACEDFF',
        blackText: '#1c0f0b',
        nutLight: '#DEB19A',
        nut: '#ab6e50',
        nutDark: '#C4B3A4',
        dark: '#191923',
        light: '#EFEFEF',
        primary: '#89BBFE',
        secondary: '#6F8AB7',
        discrete: '#50463D',
      },
    },
  },
  plugins: [],
}

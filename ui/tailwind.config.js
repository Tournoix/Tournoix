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
        intrepid: ['Intrepid', 'sans-serif'],
      },

      colors: {
        transparent: 'transparent',
        current: 'currentColor',
        highlight: '#ACEDFF',
        blackText: '#1c0f0b',
        nutDark: '#DEB19A',
        nut: '#a07253',
        nutLight: '#c8b39c',
        nutLighter: '#E6DED6',
        dark: '#191923',
        light: '#EFEFEF',
        primary: '#89BBFE',
        secondary: '#6F8AB7',
        discrete: '#50463D',
      },
    },
  },
  plugins: [],
  safelist: [
    {
      pattern: /grid-cols-+/
    },
    {
      pattern: /grid-rows-+/
    },
    {
      pattern: /py-.+/
    }
  ]
}

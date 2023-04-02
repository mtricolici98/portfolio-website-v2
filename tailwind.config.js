/** @type {import('tailwindcss').Config} */
module.exports = {
 content: [
    './src/**/*.{html,rs}',
    './index.html',
  ],
  theme: {
    extend: {
      saturate: {
        25: '.25',
      }
    },
  },
  plugins: [
  ]
}

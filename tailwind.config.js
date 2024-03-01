/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["views/templates/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}


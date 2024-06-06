/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["templates/**/*.html"],
  theme: {
    extend: {
      blur: {
        xxs: '1px',
        xs: '2px',
      },
      colors: {
      },
      fontFamily: {
        helloKetta: ["HelloKetta", "sans-serif"],
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}


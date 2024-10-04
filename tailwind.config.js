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
        'legal-pad': "#f8f0b4",
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}


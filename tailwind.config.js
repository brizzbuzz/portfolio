/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["views/templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        matrix: {
          50: "#8AEBAD",
          100: "#22C55E",
          200: "#1EAE53",
          300: "#1A9848",
          400: "#16833E",
          500: "#136D34",
          600: "#0F5729",
          700: "#0B411F",
          800: "#072C15",
          900: "#04160A",
          950: "#020D06"
        }
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}


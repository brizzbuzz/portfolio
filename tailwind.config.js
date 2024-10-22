/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/**/*.{html,rs}",
  ],
  theme: {
    extend: {
      colors: {
        // Core palette inspired by handmade paper and natural elements
         amethyst: {
          50: '#F6F4F8',   // Moonlit mist
          100: '#EBE7F1',  // Lavender fog
          200: '#D5CCE4',  // Twilight haze
          300: '#BEB0D6',  // Mystic dusk
          400: '#A794C7',  // Ethereal violet
          500: '#8F77B7',  // Spirit crystal
          600: '#775D9F',  // Deep crystal
          700: '#5F4687',  // Twilight depths
          800: '#48336E',  // Crystal shadow
          900: '#312155',  // Mystic night
        },
        parchment: {
          50: '#FDFBF7',  // Lightest ivory
          100: '#F5F1E6',  // Soft cream
          200: '#EBE5D4',  // Warm paper
          300: '#DFD5BE',  // Aged parchment
          400: '#CFC3A7',  // Textured ivory
          500: '#BFB193',  // Antique paper
          600: '#A69C7E',  // Aged manuscript
          700: '#8C8468',  // Weathered paper
          800: '#726C53',  // Deep parchment
          900: '#5A543F',  // Rich ancient paper
        },
        // Atmospheric mist and garden elements
        mist: {
          50: '#F7FAFA',   // Morning fog
          100: '#E3ECEC',  // Light mist
          200: '#D1DFDF',  // Cool breeze
          300: '#B3CCCC',  // Gentle haze
          400: '#95B8B8',  // Soft cloud
          500: '#78A3A3',  // Tranquil air
          600: '#5C8989',  // Deepening mist
          700: '#476A6A',  // Evening fog
          800: '#334D4D',  // Night air
          900: '#1F2E2E',  // Deep twilight
        },
        // Organic garden accents
        sage: {
          50: '#F2F5F3',   // Morning dew
          100: '#E0E7E2',  // Fresh leaf
          200: '#C5D1C8',  // Garden sage
          300: '#A7B9AB',  // Mature leaf
          400: '#89A18E',  // Deep foliage
          500: '#6B8871',  // Forest shadow
          600: '#526A56',  // Evening green
          700: '#3E503F',  // Deep moss
          800: '#2A362B',  // Forest depth
          900: '#161D16',  // Night garden
        },
        // Warm earth tones
        terra: {
          50: '#FAF6F3',   // Sandy light
          100: '#E9DED6',  // Soft soil
          200: '#D8C5B9',  // Warm earth
          300: '#C7AC9C',  // Clay pot
          400: '#B6937F',  // Rich terra
          500: '#A57A62',  // Deep earth
          600: '#8A6451',  // Ancient clay
          700: '#6F4E40',  // Dark soil
          800: '#54392F',  // Deep earth
          900: '#39251E',  // Rich soil
        }
      },
      fontFamily: {
        serif: ['Crimson Pro', 'serif'],
        sans: ['Inter', 'sans-serif'],
        // Optional: Add a handwriting font for special elements
        handwritten: ['Caveat', 'cursive'],
      },
      boxShadow: {
        'soft': '0 2px 15px rgba(0, 0, 0, 0.05)',
        'warm': '0 2px 15px rgba(166, 156, 126, 0.15)',
      },
      opacity: {
        '15': '0.15',
        '85': '0.85',
      }
    },
  },
  plugins: [],
}

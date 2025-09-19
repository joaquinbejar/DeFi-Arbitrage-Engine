/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          green: '#00D4AA',
          red: '#FF6B6B',
          blue: '#4ECDC4',
        },
        dark: {
          bg: '#0F0F23',
          card: '#1A1A2E',
          border: '#16213E',
        }
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}


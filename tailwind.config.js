/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        background: "#000000",
        surface: {
          DEFAULT: "#1c1c1e",
          secondary: "#2c2c2e",
          tertiary: "#3a3a3c",
        },
        nothing: {
          red: "#d71920",
          darkRed: "#7f1d1d",
          grey: "#8e8e93",
          lightGrey: "#a1a1aa",
          card: "#1c1c1e",
          subCard: "#252528",
        },
      },
      fontFamily: {
        ndot: ["NDot57", "monospace"],
        headline: ["NType82", "Georgia", "serif"],
        sans: ["SpaceGrotesk", "sans-serif"],
        mono: ["RobotoMono", "monospace"],
      },
      borderRadius: {
        "3xl": "24px",
        "4xl": "32px",
      },
    },
  },
  plugins: [],
};

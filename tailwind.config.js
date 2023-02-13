/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        night: "#131515",
        jet: "#2B2C28",
        persian: "#339989",
        tiffany: "#7DE2D1",
        snow: "#FFFAFB",
      },
      keyframes: {
        line: {
          "0%": {
            width: "0%",
          },
          "100%": {
            width: "100%",
          },
        },
      },
      animation: {
        line: "line 200ms both",
      },
    },
  },
  plugins: [],
};

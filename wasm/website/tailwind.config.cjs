/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    fontSize: {
      title: "2rem",
      paragraph: "1.1rem",
    },
    colors: {
      "nord-0": "#2e3440",
      "nord-1": "#3b4252",
      "nord-2": "#434c5e",
      "nord-3": "#4c566a",
      "nord-4": "#d8dee9",
      "nord-5": "#e5e9f0",
      "nord-6": "#eceff4",
      "nord-7": "#8fbcbb",
      "nord-8": "#88c0d0",
      "nord-9": "#81a1c1",
      "nord-10": "#5e81ac",
      "nord-11": "#bf616a",
      "nord-12": "#d08770",
      "nord-13": "#ebcb8b",
      "nord-14": "#a3be8c",
      "nord-15": "#b48ead",
    },
    extend: {},
  },
  plugins: [],
};

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        'sans': ['IBM Plex Sans'],
        'mono': ['IBM Plex Mono']
      }
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["dark"], 
  },
}


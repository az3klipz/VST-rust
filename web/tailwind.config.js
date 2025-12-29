/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{html,js}", "./index.html"],
    theme: {
        extend: {
            colors: {
                antigravity: {
                    dark: '#0f0f14',
                    panel: '#15151a',
                    accent: '#00ffc8',
                    glow: 'rgba(0, 255, 200, 0.4)',
                }
            },
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
            }
        },
    },
    plugins: [],
}

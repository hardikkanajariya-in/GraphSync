import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: [
          "Inter",
          "Segoe UI",
          "system-ui",
          "-apple-system",
          "sans-serif",
        ],
      },
      colors: {
        surface: "#fafafa",
        ink: "#111827",
        muted: "#6b7280",
        border: "#e5e7eb",
        accent: "#2563eb",
        success: "#059669",
        warning: "#d97706",
        danger: "#dc2626",
      },
    },
  },
  plugins: [],
} satisfies Config;

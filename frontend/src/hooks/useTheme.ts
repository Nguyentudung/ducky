import { useEffect } from "react";
import type { ThemePreference } from "../types/api";

export function useTheme(preference: ThemePreference) {
  useEffect(() => {
    const root = document.documentElement;
    const media = window.matchMedia("(prefers-color-scheme: dark)");

    const apply = () => {
      const isDark =
        preference === "dark" || (preference === "system" && media.matches);
      root.classList.toggle("dark", isDark);
    };

    apply();

    if (preference === "system") {
      media.addEventListener("change", apply);
      return () => media.removeEventListener("change", apply);
    }
    return undefined;
  }, [preference]);
}

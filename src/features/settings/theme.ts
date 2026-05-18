import type { ThemeOption } from "./types";

function resolveTheme(option: ThemeOption): "light" | "dark" {
  if (option === "system") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  }
  return option;
}

export function applyTheme(option: ThemeOption): void {
  const root = document.documentElement;
  root.classList.add("theme-transition");
  root.setAttribute("data-theme", resolveTheme(option));
  setTimeout(() => root.classList.remove("theme-transition"), 400);
}

let systemListener: (() => void) | null = null;

export function watchSystemTheme(option: ThemeOption): () => void {
  if (systemListener) {
    systemListener();
    systemListener = null;
  }

  if (option !== "system") return () => {};

  const mql = window.matchMedia("(prefers-color-scheme: dark)");
  const handler = () => applyTheme("system");
  mql.addEventListener("change", handler);

  const cleanup = () => {
    mql.removeEventListener("change", handler);
    systemListener = null;
  };
  systemListener = cleanup;
  return cleanup;
}

export function applyFontFamily(fontFamily: string): void {
  const root = document.documentElement;
  const defaultBody = `"Noto Sans SC", "Source Han Sans SC", system-ui, sans-serif`;
  if (fontFamily) {
    root.style.setProperty("--font-body", `"${fontFamily}", ${defaultBody}`);
  } else {
    root.style.removeProperty("--font-body");
  }
}

export function applyAppFontSize(fontSize: number): void {
  const root = document.documentElement;
  root.style.setProperty("--app-font-size", `${fontSize}px`);
}

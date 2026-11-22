import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useLocale } from "../localization/LocaleProvider";
import { IconButton } from "./Controls";

/** Sustituye la barra nativa para mantener juntos los tres controles de ventana. */
export function WindowTitlebar() {
  const { t } = useLocale();
  const [failed, setFailed] = useState(false);

  async function run(action: "minimize" | "toggleMaximize" | "close") {
    setFailed(false);
    try {
      if (action === "close") {
        await invoke("close_desktop_window");
      } else {
        await getCurrentWindow()[action]();
      }
    } catch {
      setFailed(true);
    }
  }

  return (
    <header className="dashboard-titlebar" data-tauri-drag-region>
      <span className="dashboard-titlebar__title" data-tauri-drag-region>Drasus Engine</span>
      {failed && <span className="dashboard-titlebar__error" role="alert">{t("window.action-error")}</span>}
      <div aria-label={t("window.controls")} className="dashboard-titlebar__controls" role="group">
        <IconButton
          className="dashboard-titlebar__button"
          icon="minimize"
          label={t("window.minimize")}
          onClick={() => void run("minimize")}
        />
        <IconButton
          className="dashboard-titlebar__button"
          icon="maximize"
          label={t("window.maximize-restore")}
          onClick={() => void run("toggleMaximize")}
        />
        <IconButton
          className="dashboard-titlebar__button dashboard-titlebar__button--close"
          icon="close"
          label={t("window.close")}
          onClick={() => void run("close")}
        />
      </div>
    </header>
  );
}

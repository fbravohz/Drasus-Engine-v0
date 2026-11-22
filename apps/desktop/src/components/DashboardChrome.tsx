import type { MessageKey } from "../localization/catalog";
import { useLocale } from "../localization/LocaleProvider";
import { Icon, type IconName } from "./Icon";
import { IconButton } from "./Controls";
import { useCoreStatus } from "../features/dashboard/useCoreStatus";
import type { CoreConnectionPhase } from "../bridge/core";

type SidebarProps = {
  pinned: boolean;
  onPinnedChange: () => void;
};

const navigationItems = [
  { key: "navigation.dashboard", icon: "home", current: true },
  { key: "navigation.canvas", icon: "canvas", current: false },
  { key: "navigation.strategies", icon: "layers", current: false },
  { key: "navigation.data", icon: "data", current: false },
] as const satisfies readonly { key: MessageKey; icon: IconName; current: boolean }[];

const coreStatusMessageKeys = {
  starting: "dashboard.core-connecting",
  connected: "dashboard.core-connected",
  reconnecting: "dashboard.core-reconnecting",
  error: "dashboard.core-error",
} as const satisfies Record<CoreConnectionPhase, MessageKey>;

const coreStatusSymbols = {
  starting: "◌",
  connected: "●",
  reconnecting: "↻",
  error: "!",
} as const satisfies Record<CoreConnectionPhase, string>;

export function Sidebar({ pinned, onPinnedChange }: SidebarProps) {
  const { t } = useLocale();
  return (
    <aside aria-label={t("navigation.primary-label")} className={`dashboard-sidebar${pinned ? " dashboard-sidebar--pinned" : ""}`}>
      <div className="dashboard-sidebar__surface">
        <div className="dashboard-brand">
          <span aria-hidden="true" className="dashboard-brand__mark">
            <svg fill="none" viewBox="0 0 32 32">
              <path d="M3 7 14 16 3 25zM29 7 18 16l11 9z" fill="currentColor" />
              <path d="m16 2 13 7-13 7L3 9zM16 16l13 7-13 7-13-7z" fill="#777" />
            </svg>
          </span>
          <span className="dashboard-brand__text">Drasus Engine</span>
        </div>
        <nav aria-label={t("navigation.sections-label")} className="dashboard-sidebar__nav">
          {navigationItems.map((item) => (
            <button
              aria-current={item.current ? "page" : undefined}
              className="dashboard-rail-button"
              disabled={!item.current}
              key={item.key}
              type="button"
            >
              <Icon className="size-[20px] shrink-0" name={item.icon} />
              <span>{t(item.key)}</span>
            </button>
          ))}
        </nav>
        <div className="dashboard-sidebar__bottom">
          <IconButton
            aria-pressed={pinned}
            className="dashboard-pin-button"
            icon="pin"
            label={t(pinned ? "navigation.unpin" : "navigation.pin")}
            onClick={onPinnedChange}
          />
        </div>
      </div>
    </aside>
  );
}

export function DashboardHeader() {
  const { locale, locales, setLocale, t } = useLocale();
  const { status, retry, retrying } = useCoreStatus();
  const statusKey = coreStatusMessageKeys[status.phase];
  const statusSymbol = coreStatusSymbols[status.phase];

  return (
    <header className="dashboard-header">
      <div className="dashboard-header__title">
        <h1>{t("dashboard.title")}</h1>
        <p>{t("dashboard.subtitle")}</p>
      </div>
      <div className="dashboard-header__actions">
        <div aria-atomic="true" aria-live="polite" className={`dashboard-core-status dashboard-core-status--${status.phase}`} role="status">
          <span aria-hidden="true" className="dashboard-core-status__symbol">{statusSymbol}</span>
          <span>{t(statusKey)}</span>
        </div>
        {status.phase === "error" && (
          <button
            className="dashboard-core-retry"
            disabled={retrying || status.isRecovering}
            onClick={() => void retry()}
            type="button"
          >
            {t("dashboard.core-retry")}
          </button>
        )}
        <label className="dashboard-locale-setting" htmlFor="dashboard-locale">
          <span>{t("locale.selector-label")}</span>
          <select
            className="dashboard-locale-select"
            id="dashboard-locale"
            onChange={(event) => setLocale(event.currentTarget.value)}
            value={locale}
          >
            {locales.map((option) => <option key={option.code} value={option.code}>{option.label}</option>)}
          </select>
        </label>
      </div>
    </header>
  );
}

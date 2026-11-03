import { useState } from "react";
import { DashboardHeader, Sidebar } from "../../components/DashboardChrome";
import { WidgetEmptyState } from "../../components/DashboardWidgets";
import { WidgetCard } from "../../components/WidgetCard";
import { dashboardWidgets, widgetOrder } from "../../data/dashboard";
import { useLocale } from "../../localization/LocaleProvider";

export function Dashboard() {
  const { t } = useLocale();
  const [pinned, setPinned] = useState(false);

  return (
    <div className="dashboard-app font-sans text-dashboard-text">
      <div className={`dashboard-layout${pinned ? " dashboard-layout--pinned" : ""}`}>
        <Sidebar onPinnedChange={() => setPinned((current) => !current)} pinned={pinned} />
        <main className="dashboard-workspace">
          <DashboardHeader />
          <div aria-label={t("dashboard.widget-list")} className="dashboard-grid">
            {widgetOrder.map((id) => {
              const widget = dashboardWidgets[id];
              return (
                <WidgetCard
                  id={id}
                  key={id}
                  layout={widget.layout}
                  subtitle={t(widget.subtitleKey)}
                  title={t(widget.titleKey)}
                >
                  <WidgetEmptyState
                    description={t(widget.emptyDescriptionKey)}
                    title={t(widget.emptyTitleKey)}
                  />
                </WidgetCard>
              );
            })}
          </div>
        </main>
      </div>
    </div>
  );
}

import type { ReactNode } from "react";
import type { WidgetId, WidgetLayout } from "../data/dashboard";

type WidgetCardProps = {
  children: ReactNode;
  id: WidgetId;
  layout: WidgetLayout;
  subtitle: string;
  title: string;
};

export function WidgetCard({ children, id, layout, subtitle, title }: WidgetCardProps) {
  return (
    <section aria-labelledby={`dashboard-widget-title-${id}`} className="dashboard-widget" data-layout={layout}>
      <header className="dashboard-widget__header">
        <h2 className="dashboard-widget__title" id={`dashboard-widget-title-${id}`}>{title}</h2>
        <p className="dashboard-widget__subtitle">{subtitle}</p>
      </header>
      <div className="dashboard-widget__content">{children}</div>
    </section>
  );
}

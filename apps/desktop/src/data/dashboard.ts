import type { MessageKey } from "../localization/catalog";

export type WidgetId = "results" | "period" | "research" | "portfolios" | "risk";
export type WidgetLayout = "results" | "period" | "standard";

export type WidgetDefinition = {
  id: WidgetId;
  titleKey: MessageKey;
  subtitleKey: MessageKey;
  layout: WidgetLayout;
  emptyTitleKey: MessageKey;
  emptyDescriptionKey: MessageKey;
};

export const dashboardWidgets: Record<WidgetId, WidgetDefinition> = {
  results: {
    id: "results",
    titleKey: "widget.results.title",
    subtitleKey: "widget.results.subtitle",
    layout: "results",
    emptyTitleKey: "widget.results.empty-title",
    emptyDescriptionKey: "widget.results.empty-description",
  },
  period: {
    id: "period",
    titleKey: "widget.period.title",
    subtitleKey: "widget.period.subtitle",
    layout: "period",
    emptyTitleKey: "widget.period.empty-title",
    emptyDescriptionKey: "widget.period.empty-description",
  },
  research: {
    id: "research",
    titleKey: "widget.research.title",
    subtitleKey: "widget.research.subtitle",
    layout: "standard",
    emptyTitleKey: "widget.research.empty-title",
    emptyDescriptionKey: "widget.research.empty-description",
  },
  portfolios: {
    id: "portfolios",
    titleKey: "widget.portfolios.title",
    subtitleKey: "widget.portfolios.subtitle",
    layout: "standard",
    emptyTitleKey: "widget.portfolios.empty-title",
    emptyDescriptionKey: "widget.portfolios.empty-description",
  },
  risk: {
    id: "risk",
    titleKey: "widget.risk.title",
    subtitleKey: "widget.risk.subtitle",
    layout: "standard",
    emptyTitleKey: "widget.risk.empty-title",
    emptyDescriptionKey: "widget.risk.empty-description",
  },
};

export const widgetOrder: readonly WidgetId[] = ["results", "period", "research", "portfolios", "risk"];

import { Icon } from "./Icon";

type WidgetEmptyStateProps = {
  description: string;
  title: string;
};

export function WidgetEmptyState({ description, title }: WidgetEmptyStateProps) {
  return (
    <div className="dashboard-empty-state">
      <span aria-hidden="true" className="dashboard-empty-state__icon"><Icon name="info" /></span>
      <div>
        <strong>{title}</strong>
        <p>{description}</p>
      </div>
    </div>
  );
}

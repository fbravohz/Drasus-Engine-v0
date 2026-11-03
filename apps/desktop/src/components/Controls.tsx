import type { ButtonHTMLAttributes } from "react";
import { Icon, type IconName } from "./Icon";

type IconButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  icon: IconName;
  label: string;
};

export function IconButton({ icon, label, className = "", title, ...props }: IconButtonProps) {
  return (
    <button
      aria-label={label}
      className={`dashboard-icon-button ${className}`.trim()}
      title={title ?? label}
      type="button"
      {...props}
    >
      <Icon className="size-[18px]" name={icon} />
    </button>
  );
}

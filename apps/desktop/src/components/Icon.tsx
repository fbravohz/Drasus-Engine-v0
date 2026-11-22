import type { ReactNode } from "react";

export type IconName =
  | "home"
  | "canvas"
  | "layers"
  | "data"
  | "pin"
  | "info"
  | "minimize"
  | "maximize"
  | "close";

const iconShapes: Record<IconName, ReactNode> = {
  home: <><path d="m4 10 8-6 8 6v9H4z" /><path d="M9 19v-6h6v6" /></>,
  canvas: <><rect x="3" y="4" width="6" height="6" rx="1" /><rect x="15" y="14" width="6" height="6" rx="1" /><path d="M9 7h4a3 3 0 0 1 3 3v4" /></>,
  layers: <><path d="m12 3 9 5-9 5-9-5z" /><path d="m3 12 9 5 9-5M3 16l9 5 9-5" /></>,
  data: <><ellipse cx="12" cy="5" rx="8" ry="3" /><path d="M4 5v7c0 1.7 3.6 3 8 3s8-1.3 8-3V5M4 12v7c0 1.7 3.6 3 8 3s8-1.3 8-3v-7" /></>,
  pin: <><path d="m8 4 8 0-1 5 3 3v2H6v-2l3-3zM12 14v7" /></>,
  info: <><circle cx="12" cy="12" r="9" /><path d="M12 11v5M12 8h.01" /></>,
  minimize: <path d="M5 18h14" />,
  maximize: <rect x="5" y="5" width="14" height="14" rx="1" />,
  close: <path d="M5 5 19 19M19 5 5 19" />,
};

type IconProps = {
  name: IconName;
  className?: string;
};

export function Icon({ name, className }: IconProps) {
  return (
    <svg
      aria-hidden="true"
      className={className}
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="1.6"
      viewBox="0 0 24 24"
    >
      {iconShapes[name]}
    </svg>
  );
}

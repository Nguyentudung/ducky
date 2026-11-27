import type { LucideIcon } from "lucide-react";
import type { CSSProperties } from "react";

interface IconProps {
  icon: LucideIcon;
  size?: number;
  strokeWidth?: number;
  className?: string;
  style?: CSSProperties;
}

export function Icon({
  icon: LucideComponent,
  size = 16,
  strokeWidth = 1.75,
  className,
  style,
}: IconProps) {
  return (
    <LucideComponent
      size={size}
      strokeWidth={strokeWidth}
      className={className}
      style={style}
      aria-hidden="true"
    />
  );
}

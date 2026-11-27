import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/utils";

export const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 font-medium transition-all duration-150 cursor-pointer select-none disabled:opacity-45 disabled:cursor-not-allowed whitespace-nowrap active:scale-[0.98] focus-visible:outline-none focus-visible:ring-3 focus-visible:ring-[var(--ring-focus)]",
  {
    variants: {
      variant: {
        primary:
          "rounded-full bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)] shadow-[0_1px_2px_rgba(0,0,0,0.12)]",
        secondary:
          "rounded-[10px] border border-[var(--border-subtle)] bg-[var(--bg-surface)] text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] shadow-[var(--shadow-sm)]",
        ghost:
          "rounded-[10px] bg-transparent text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)]",
        danger:
          "rounded-full bg-[var(--danger)] text-white hover:opacity-90 shadow-[0_1px_2px_rgba(0,0,0,0.12)]",
      },
      size: {
        sm: "h-9 px-3.5 text-[13px]",
        md: "h-10 px-4 text-[14px]",
        lg: "h-11 px-5 text-[14px]",
        icon: "h-9 w-9 p-0",
      },
    },
    compoundVariants: [
      {
        variant: "primary",
        size: "sm",
        className: "px-4",
      },
      {
        variant: "primary",
        size: "md",
        className: "px-5",
      },
      {
        variant: "primary",
        size: "lg",
        className: "px-6",
      },
      {
        variant: "danger",
        size: "md",
        className: "px-5",
      },
    ],
    defaultVariants: {
      variant: "secondary",
      size: "md",
    },
  },
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, type = "button", ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    return (
      <Comp
        ref={ref}
        type={asChild ? undefined : type}
        className={cn(buttonVariants({ variant, size, className }))}
        {...props}
      />
    );
  },
);
Button.displayName = "Button";

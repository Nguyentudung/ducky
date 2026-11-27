import * as React from "react";
import { AlertCircle, Loader2 } from "lucide-react";
import { cn } from "../../lib/utils";
import { Button } from "./button";
import type { ProblemDifficulty, ProblemStatus } from "../../types/api";
import type { TranslationDict } from "../../i18n/translations";

export const Input = React.forwardRef<
  HTMLInputElement,
  React.InputHTMLAttributes<HTMLInputElement>
>(({ className, ...props }, ref) => (
  <input
    ref={ref}
    className={cn(
      "h-10 w-full rounded-[10px] border border-[var(--border-subtle)] bg-[var(--bg-surface)] px-3.5 text-[14px] text-[var(--text-primary)] placeholder:text-[var(--text-muted)] transition-shadow focus:border-[var(--accent)] focus:ring-3 focus:ring-[var(--ring-focus)] focus:outline-none disabled:opacity-50",
      className,
    )}
    {...props}
  />
));
Input.displayName = "Input";

export const Textarea = React.forwardRef<
  HTMLTextAreaElement,
  React.TextareaHTMLAttributes<HTMLTextAreaElement>
>(({ className, ...props }, ref) => (
  <textarea
    ref={ref}
    className={cn(
      "w-full rounded-[10px] border border-[var(--border-subtle)] bg-[var(--bg-surface)] px-3.5 py-2.5 text-[14px] leading-relaxed text-[var(--text-primary)] placeholder:text-[var(--text-muted)] transition-shadow focus:border-[var(--accent)] focus:ring-3 focus:ring-[var(--ring-focus)] focus:outline-none disabled:opacity-50",
      className,
    )}
    {...props}
  />
));
Textarea.displayName = "Textarea";

export function Card({
  className,
  children,
}: {
  className?: string;
  children: React.ReactNode;
}) {
  return (
    <div
      className={cn(
        "rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-surface)]",
        className,
      )}
    >
      {children}
    </div>
  );
}

export function Badge({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <span
      className={cn(
        "inline-flex items-center rounded-md bg-[var(--bg-subtle)] px-2 py-0.5 text-[12px] font-medium text-[var(--text-secondary)]",
        className,
      )}
    >
      {children}
    </span>
  );
}

export function StatusBadge({
  status,
  t,
}: {
  status: ProblemStatus;
  t: TranslationDict;
}) {
  const styles: Record<
    ProblemStatus,
    { bg: string; fg: string; dot: string }
  > = {
    not_started: {
      bg: "var(--bg-subtle)",
      fg: "var(--text-secondary)",
      dot: "var(--text-muted)",
    },
    in_progress: {
      bg: "var(--info-soft)",
      fg: "var(--info)",
      dot: "var(--info)",
    },
    solved: {
      bg: "var(--success-soft)",
      fg: "var(--success)",
      dot: "var(--success)",
    },
    review: {
      bg: "var(--warning-soft)",
      fg: "var(--warning)",
      dot: "var(--warning)",
    },
  };
  const s = styles[status] ?? styles.not_started;
  return (
    <span
      className="inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-[12px] font-medium"
      style={{ backgroundColor: s.bg, color: s.fg }}
    >
      <span
        className="h-1.5 w-1.5 rounded-full"
        style={{ backgroundColor: s.dot }}
        aria-hidden="true"
      />
      {t.statusLabels[status] ?? status}
    </span>
  );
}

export function DifficultyBadge({
  difficulty,
  t,
}: {
  difficulty: ProblemDifficulty;
  t: TranslationDict;
}) {
  const styles: Record<ProblemDifficulty, { bg: string; fg: string }> = {
    easy: { bg: "var(--success-soft)", fg: "var(--success)" },
    medium: { bg: "var(--warning-soft)", fg: "var(--warning)" },
    hard: { bg: "var(--danger-soft)", fg: "var(--danger)" },
    custom: { bg: "var(--bg-subtle)", fg: "var(--text-secondary)" },
  };
  const s = styles[difficulty] ?? styles.custom;
  return (
    <span
      className="inline-flex items-center rounded-md px-2 py-0.5 text-[12px] font-medium"
      style={{ backgroundColor: s.bg, color: s.fg }}
    >
      {t.difficultyLabels[difficulty] ?? difficulty}
    </span>
  );
}

export function EmptyState({
  icon,
  title,
  description,
  action,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
  action?: React.ReactNode;
}) {
  return (
    <div className="flex flex-col items-center justify-center rounded-xl border border-dashed border-[var(--border-strong)] bg-[var(--bg-surface)]/60 px-6 py-14 text-center">
      <div className="mb-3.5 flex h-12 w-12 items-center justify-center rounded-xl bg-[var(--bg-subtle)] text-[var(--text-secondary)]">
        {icon}
      </div>
      <h3 className="text-[16px] font-semibold tracking-tight text-[var(--text-primary)]">
        {title}
      </h3>
      <p className="mt-1.5 max-w-sm text-[13px] leading-relaxed text-[var(--text-secondary)]">
        {description}
      </p>
      {action ? <div className="mt-5">{action}</div> : null}
    </div>
  );
}

export function ErrorBanner({
  message,
  onRetry,
  retryLabel = "Retry",
}: {
  message: string;
  onRetry?: () => void;
  retryLabel?: string;
}) {
  return (
    <div
      role="alert"
      className="flex flex-wrap items-center justify-between gap-3 rounded-xl border px-4 py-3 text-[13px]"
      style={{
        backgroundColor: "var(--danger-soft)",
        borderColor: "var(--danger)",
        color: "var(--danger)",
      }}
    >
      <div className="flex items-center gap-2">
        <AlertCircle size={16} strokeWidth={1.8} className="shrink-0" />
        <span>{message}</span>
      </div>
      {onRetry ? (
        <Button size="sm" variant="secondary" onClick={onRetry}>
          {retryLabel}
        </Button>
      ) : null}
    </div>
  );
}

export function LoadingState({ label }: { label: string }) {
  return (
    <div className="flex items-center justify-center gap-2.5 py-20 text-[14px] text-[var(--text-muted)]">
      <Loader2 size={17} strokeWidth={2} className="animate-spin" />
      <span>{label}</span>
    </div>
  );
}

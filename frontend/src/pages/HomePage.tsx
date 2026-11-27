import { useCallback, useEffect, useState } from "react";
import {
  ArrowRight,
  ChevronRight,
  Clock,
  Code2,
  FolderOpen,
  History,
  Plus,
  Trash2,
} from "lucide-react";
import { api } from "../services/api";
import { Icon } from "../components/Icon";
import {
  Badge,
  Button,
  Card,
  Dialog,
  DifficultyBadge,
  EmptyState,
  ErrorBanner,
  LoadingState,
  StatusBadge,
} from "../components/ui";
import type { Route } from "../components/Layout";
import type { TranslationDict } from "../i18n/translations";
import type { DashboardResponse, Problem } from "../types/api";
import { useToast } from "../hooks/useToast";

interface HomePageProps {
  t: TranslationDict;
  refreshKey: number;
  onNavigate: (r: Route) => void;
  onNewProblem: () => void;
  onOpenWorkspace: () => void;
}

export function HomePage({
  t,
  refreshKey,
  onNavigate,
  onNewProblem,
  onOpenWorkspace,
}: HomePageProps) {
  const { pushToast } = useToast();
  const [data, setData] = useState<DashboardResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [confirmClearOpen, setConfirmClearOpen] = useState(false);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await api.dashboard();
      setData(res);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load dashboard");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load, refreshKey]);

  const handleClearRecent = async () => {
    try {
      await api.clearRecent();
      setConfirmClearOpen(false);
      pushToast(t.toastRecentCleared, "success");
      await load();
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to clear history",
        "error",
      );
    }
  };

  if (loading && !data) return <LoadingState label={t.loading} />;
  if (error && !data) {
    return (
      <ErrorBanner
        message={error}
        onRetry={() => void load()}
        retryLabel={t.retry}
      />
    );
  }
  if (!data) return null;

  const { stats } = data;

  const metrics = [
    { label: t.totalProblems, value: stats.total, color: "var(--text-primary)" },
    { label: t.solved, value: stats.solved, color: "var(--success)" },
    { label: t.inProgress, value: stats.in_progress, color: "var(--info)" },
    { label: t.review, value: stats.review, color: "var(--warning)" },
  ];

  const renderProblemRow = (problem: Problem) => (
    <button
      key={problem.id}
      type="button"
      onClick={() => onNavigate({ page: "problem", id: problem.id })}
      className="group flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-[var(--bg-subtle)]/60 cursor-pointer"
    >
      <div className="min-w-0 flex-1">
        <div className="flex items-center gap-2">
          <span className="truncate text-[13px] font-medium text-[var(--text-primary)] group-hover:text-[var(--accent)] transition-colors">
            {problem.title}
          </span>
          {problem.platform ? (
            <Badge className="shrink-0">{problem.platform}</Badge>
          ) : null}
        </div>
        <div className="mt-1.5 flex flex-wrap items-center gap-1.5">
          <StatusBadge status={problem.status} t={t} />
          <DifficultyBadge difficulty={problem.difficulty} t={t} />
          {problem.languages.length > 0 ? (
            <span className="ml-0.5 font-mono text-[11px] text-[var(--text-muted)]">
              {problem.languages.join(" · ")}
            </span>
          ) : null}
        </div>
      </div>
      <Icon
        icon={ChevronRight}
        size={15}
        className="shrink-0 text-[var(--text-muted)] transition-transform group-hover:translate-x-0.5"
      />
    </button>
  );

  return (
    <div className="space-y-7">
      {/* Header */}
      <div className="flex flex-wrap items-end justify-between gap-4">
        <div>
          <h1 className="text-xl font-semibold tracking-tight text-[var(--text-primary)]">
            {t.navHome}
          </h1>
          <p className="mt-1 text-[13px] text-[var(--text-secondary)]">
            {t.homeSubtitle}
          </p>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <Button variant="secondary" size="sm" onClick={onOpenWorkspace}>
            <Icon icon={FolderOpen} size={14} />
            {t.openWorkspace}
          </Button>
          {stats.total > 0 ? (
            <Button
              variant="secondary"
              size="sm"
              onClick={() => onNavigate({ page: "problems" })}
            >
              <Icon icon={Code2} size={14} />
              {t.browseProblems}
            </Button>
          ) : null}
          <Button variant="primary" size="sm" onClick={onNewProblem}>
            <Icon icon={Plus} size={14} />
            {t.newProblem}
          </Button>
        </div>
      </div>

      {stats.total === 0 ? (
        <EmptyState
          icon={<Icon icon={Code2} size={20} />}
          title={t.homeEmptyTitle}
          description={t.homeEmptyDesc}
          action={
            <div className="flex flex-wrap items-center justify-center gap-2">
              <Button variant="primary" onClick={onNewProblem}>
                <Icon icon={Plus} size={15} />
                {t.createFirstProblem}
              </Button>
              <Button variant="secondary" onClick={onOpenWorkspace}>
                <Icon icon={FolderOpen} size={15} />
                {t.openWorkspace}
              </Button>
            </div>
          }
        />
      ) : (
        <>
          {/* Unified Practice Overview Strip */}
          <Card className="overflow-hidden">
            <div className="grid grid-cols-2 divide-y divide-[var(--border-subtle)] sm:grid-cols-4 sm:divide-x sm:divide-y-0">
              {metrics.map((m) => (
                <div key={m.label} className="px-5 py-4">
                  <div className="text-[11px] font-medium uppercase tracking-wider text-[var(--text-muted)]">
                    {m.label}
                  </div>
                  <div
                    className="mt-1 text-2xl font-semibold tracking-tight tabular-nums"
                    style={{ color: m.color }}
                  >
                    {m.value}
                  </div>
                </div>
              ))}
            </div>
            <div className="border-t border-[var(--border-subtle)] bg-[var(--bg-canvas)]/50 px-5 py-3">
              <div className="flex items-center justify-between text-xs">
                <span className="font-medium text-[var(--text-secondary)]">
                  {t.completion}
                </span>
                <span className="font-mono text-xs font-medium tabular-nums text-[var(--text-primary)]">
                  {stats.completion_percent}% ({stats.solved}/{stats.total})
                </span>
              </div>
              <div className="mt-2 h-1.5 w-full overflow-hidden rounded-full bg-[var(--bg-muted)]">
                <div
                  className="h-full rounded-full transition-all duration-300"
                  style={{
                    width: `${Math.min(100, Math.max(0, stats.completion_percent))}%`,
                    backgroundColor: "var(--accent)",
                  }}
                />
              </div>
            </div>
          </Card>

          {/* Activity Lists */}
          <div className="grid grid-cols-1 gap-5 lg:grid-cols-2">
            <Card className="overflow-hidden">
              <div className="flex items-center justify-between border-b border-[var(--border-subtle)] px-4 py-3">
                <h2 className="flex items-center gap-2 text-[13px] font-semibold text-[var(--text-primary)]">
                  <Icon
                    icon={Clock}
                    size={15}
                    className="text-[var(--text-muted)]"
                  />
                  {t.recentlyOpened}
                </h2>
                {stats.recent.length > 0 ? (
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => setConfirmClearOpen(true)}
                  >
                    <Icon icon={Trash2} size={13} />
                    {t.clearHistory}
                  </Button>
                ) : null}
              </div>
              {stats.recent.length === 0 ? (
                <div className="px-4 py-10 text-center">
                  <p className="text-[13px] font-medium text-[var(--text-secondary)]">
                    {t.noRecentYet}
                  </p>
                  <p className="mt-1 text-xs text-[var(--text-muted)]">
                    {t.noRecentDesc}
                  </p>
                </div>
              ) : (
                <div className="divide-y divide-[var(--border-subtle)]">
                  {stats.recent.map(renderProblemRow)}
                </div>
              )}
            </Card>

            <Card className="overflow-hidden">
              <div className="flex items-center justify-between border-b border-[var(--border-subtle)] px-4 py-3">
                <h2 className="flex items-center gap-2 text-[13px] font-semibold text-[var(--text-primary)]">
                  <Icon
                    icon={History}
                    size={15}
                    className="text-[var(--text-muted)]"
                  />
                  {t.recentlyModified}
                </h2>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => onNavigate({ page: "problems" })}
                >
                  {t.browseProblems}
                  <Icon icon={ArrowRight} size={13} />
                </Button>
              </div>
              <div className="divide-y divide-[var(--border-subtle)]">
                {stats.modified.map(renderProblemRow)}
              </div>
            </Card>
          </div>
        </>
      )}

      <Dialog
        open={confirmClearOpen}
        onClose={() => setConfirmClearOpen(false)}
        title={t.clearHistoryConfirmTitle}
        maxWidth="max-w-md"
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={() => setConfirmClearOpen(false)}>
              {t.cancel}
            </Button>
            <Button variant="danger" onClick={() => void handleClearRecent()}>
              {t.confirm}
            </Button>
          </div>
        }
      >
        <p className="text-[13px] leading-relaxed text-[var(--text-secondary)]">
          {t.clearHistoryConfirmBody}
        </p>
      </Dialog>
    </div>
  );
}

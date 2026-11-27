import { useCallback, useEffect, useState } from "react";
import {
  ChevronRight,
  Code2,
  ExternalLink,
  FolderOpen,
  Plus,
  RotateCcw,
  Search,
  X,
} from "lucide-react";
import { api } from "../services/api";
import { Icon } from "../components/Icon";
import {
  Badge,
  Button,
  Card,
  DifficultyBadge,
  EmptyState,
  ErrorBanner,
  Input,
  LoadingState,
  SelectField,
  Tooltip,
  buttonVariants,
  cn,
} from "../components/ui";
import type { Route } from "../components/Layout";
import type { TranslationDict } from "../i18n/translations";
import type { Language, Problem, ProblemQuery, ProblemStatus } from "../types/api";
import { useToast } from "../hooks/useToast";

interface ProblemsPageProps {
  t: TranslationDict;
  languages: Language[];
  refreshKey: number;
  onNavigate: (r: Route) => void;
  onNewProblem: () => void;
  onChanged: () => void;
}

export function ProblemsPage({
  t,
  languages,
  refreshKey,
  onNavigate,
  onNewProblem,
  onChanged,
}: ProblemsPageProps) {
  const { pushToast } = useToast();
  const [problems, setProblems] = useState<Problem[]>([]);
  const [platforms, setPlatforms] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [search, setSearch] = useState("");
  const [status, setStatus] = useState("");
  const [difficulty, setDifficulty] = useState("");
  const [language, setLanguage] = useState("");
  const [platform, setPlatform] = useState("");
  const [sort, setSort] = useState<NonNullable<ProblemQuery["sort"]>>("modified");

  const hasFilters = Boolean(search || status || difficulty || language || platform);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [list, dash] = await Promise.all([
        api.listProblems({
          search,
          status,
          difficulty,
          language,
          platform,
          sort,
          dir: sort === "title" ? "asc" : "desc",
        }),
        api.dashboard(),
      ]);
      setProblems(list);
      setPlatforms(dash.stats.platforms);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load problems");
    } finally {
      setLoading(false);
    }
  }, [search, status, difficulty, language, platform, sort]);

  useEffect(() => {
    const timer = window.setTimeout(() => {
      void load();
    }, search ? 150 : 0);
    return () => window.clearTimeout(timer);
  }, [load, refreshKey, search]);

  const handleQuickStatus = async (
    problem: Problem,
    nextStatus: ProblemStatus,
  ) => {
    try {
      const updated = await api.updateProblem(problem.id, {
        status: nextStatus,
      });
      setProblems((prev) =>
        prev.map((p) => (p.id === problem.id ? updated : p)),
      );
      pushToast(t.toastProblemUpdated, "success");
      onChanged();
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to update status",
        "error",
      );
    }
  };

  const handleOpenFolder = async (id: number) => {
    try {
      await api.openProblemFolder(id);
      pushToast(t.toastFolderOpened, "info");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to open folder",
        "error",
      );
    }
  };

  const resetFilters = () => {
    setSearch("");
    setStatus("");
    setDifficulty("");
    setLanguage("");
    setPlatform("");
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-wrap items-end justify-between gap-4">
        <div>
          <div className="flex items-center gap-2.5">
            <h1 className="text-xl font-semibold tracking-tight text-[var(--text-primary)]">
              {t.problemsTitle}
            </h1>
            {!loading ? (
              <Badge className="font-mono tabular-nums">{problems.length}</Badge>
            ) : null}
          </div>
          <p className="mt-1 text-[13px] text-[var(--text-secondary)]">
            {t.problemsSubtitle}
          </p>
        </div>
        <Button
          variant="primary"
          size="sm"
          className="hidden lg:inline-flex"
          onClick={onNewProblem}
        >
          <Icon icon={Plus} size={14} />
          {t.newProblem}
        </Button>
      </div>

      {/* Filter Bar */}
      <Card className="p-3.5">
        <div className="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-[1.6fr_1fr_1fr_1fr_1.3fr]">
          <div className="relative sm:col-span-2 lg:col-span-1">
            <Icon
              icon={Search}
              size={14}
              className="pointer-events-none absolute left-3.5 top-1/2 -translate-y-1/2 text-[var(--text-muted)]"
            />
            <Input
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder={t.searchPlaceholder}
              className="rounded-full pl-9 pr-8"
            />
            {search ? (
              <button
                type="button"
                onClick={() => setSearch("")}
                aria-label="Clear search"
                className="absolute right-2.5 top-1/2 -translate-y-1/2 rounded-full p-1 text-[var(--text-muted)] hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)] cursor-pointer"
              >
                <Icon icon={X} size={13} />
              </button>
            ) : null}
          </div>

          <div>
            <SelectField
              value={status}
              onValueChange={setStatus}
              aria-label={t.status}
              options={[
                { value: "", label: t.allStatuses },
                ...(
                  [
                    "not_started",
                    "in_progress",
                    "solved",
                    "review",
                  ] as ProblemStatus[]
                ).map((s) => ({
                  value: s,
                  label: t.statusLabels[s],
                })),
              ]}
            />
          </div>

          <div>
            <SelectField
              value={difficulty}
              onValueChange={setDifficulty}
              aria-label={t.difficulty}
              options={[
                { value: "", label: t.allDifficulties },
                ...(["easy", "medium", "hard", "custom"] as const).map((d) => ({
                  value: d,
                  label: t.difficultyLabels[d],
                })),
              ]}
            />
          </div>

          <div>
            <SelectField
              value={language}
              onValueChange={setLanguage}
              aria-label={t.language}
              options={[
                { value: "", label: t.allLanguages },
                ...languages.map((l) => ({
                  value: l.id,
                  label: l.name,
                })),
              ]}
            />
          </div>

          <div>
            <SelectField
              value={sort}
              onValueChange={(v) =>
                setSort(v as NonNullable<ProblemQuery["sort"]>)
              }
              aria-label={t.sortBy}
              options={[
                { value: "modified", label: t.sortModified },
                { value: "created", label: t.sortCreated },
                { value: "title", label: t.sortTitle },
                { value: "difficulty", label: t.sortDifficulty },
                { value: "status", label: t.sortStatus },
              ]}
            />
          </div>
        </div>

        {platforms.length > 0 || hasFilters ? (
          <div className="mt-3 flex flex-wrap items-center justify-between gap-2 border-t border-[var(--border-subtle)] pt-3">
            <div className="flex flex-wrap items-center gap-1.5">
              <button
                type="button"
                onClick={() => setPlatform("")}
                className={cn(
                  "rounded-full px-3 py-1 text-xs font-medium transition-colors cursor-pointer",
                  platform === ""
                    ? "bg-[var(--accent-soft)] text-[var(--accent-text)]"
                    : "text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)]",
                )}
              >
                {t.allPlatforms}
              </button>
              {platforms.map((p) => (
                <button
                  key={p}
                  type="button"
                  onClick={() => setPlatform(p === platform ? "" : p)}
                  className={cn(
                    "rounded-full px-3 py-1 text-xs font-medium transition-colors cursor-pointer",
                    platform === p
                      ? "bg-[var(--accent-soft)] text-[var(--accent-text)]"
                      : "text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)]",
                  )}
                >
                  {p}
                </button>
              ))}
            </div>
            {hasFilters ? (
              <Button size="sm" variant="ghost" onClick={resetFilters}>
                <Icon icon={RotateCcw} size={13} />
                {t.resetFilters}
              </Button>
            ) : null}
          </div>
        ) : null}
      </Card>

      {error ? (
        <ErrorBanner
          message={error}
          onRetry={() => void load()}
          retryLabel={t.retry}
        />
      ) : null}

      {loading && problems.length === 0 ? (
        <LoadingState label={t.loading} />
      ) : problems.length === 0 ? (
        <EmptyState
          icon={<Icon icon={Code2} size={20} />}
          title={hasFilters ? t.noMatchesTitle : t.noProblemsTitle}
          description={hasFilters ? t.noMatchesDesc : t.noProblemsDesc}
          action={
            hasFilters ? (
              <Button variant="secondary" onClick={resetFilters}>
                <Icon icon={RotateCcw} size={14} />
                {t.resetFilters}
              </Button>
            ) : (
              <Button variant="primary" onClick={onNewProblem}>
                <Icon icon={Plus} size={15} />
                {t.newProblem}
              </Button>
            )
          }
        />
      ) : (
        <Card className="divide-y divide-[var(--border-subtle)] overflow-hidden">
          {problems.map((problem) => (
            <div
              key={problem.id}
              className="group flex flex-col gap-3 px-5 py-3.5 transition-colors hover:bg-[var(--bg-subtle)]/40 sm:flex-row sm:items-center sm:justify-between"
            >
              <div className="min-w-0 flex-1">
                <div className="flex flex-wrap items-center gap-2">
                  <button
                    type="button"
                    onClick={() =>
                      onNavigate({ page: "problem", id: problem.id })
                    }
                    className="truncate text-left text-[14px] font-semibold text-[var(--text-primary)] transition-colors hover:text-[var(--accent)] cursor-pointer"
                  >
                    {problem.title}
                  </button>
                  {problem.identifier ? (
                    <span className="rounded-full bg-[var(--bg-subtle)] px-2 py-0.5 font-mono text-[11px] text-[var(--text-muted)]">
                      {problem.identifier}
                    </span>
                  ) : null}
                  {problem.platform ? <Badge>{problem.platform}</Badge> : null}
                  <DifficultyBadge difficulty={problem.difficulty} t={t} />
                </div>

                <div className="mt-1.5 flex flex-wrap items-center gap-2 text-xs text-[var(--text-muted)]">
                  <span className="font-mono text-[11px] text-[var(--text-secondary)]">
                    {problem.languages.join(" · ")}
                  </span>
                  {problem.tags.slice(0, 4).map((tag) => (
                    <span
                      key={tag}
                      className="rounded-full border border-[var(--border-subtle)] px-2 py-0.5 text-[11px] text-[var(--text-secondary)]"
                    >
                      #{tag}
                    </span>
                  ))}
                  <span className="hidden font-mono text-[11px] text-[var(--text-muted)] md:inline">
                    {problem.updated_at.slice(0, 10)}
                  </span>
                </div>
              </div>

              <div className="flex shrink-0 flex-wrap items-center gap-2">
                <div className="w-36">
                  <SelectField
                    size="sm"
                    aria-label={`${t.status} for ${problem.title}`}
                    value={problem.status}
                    onValueChange={(val) =>
                      void handleQuickStatus(problem, val as ProblemStatus)
                    }
                    options={(
                      [
                        "not_started",
                        "in_progress",
                        "solved",
                        "review",
                      ] as ProblemStatus[]
                    ).map((s) => ({
                      value: s,
                      label: t.statusLabels[s],
                    }))}
                  />
                </div>

                {problem.url ? (
                  <Tooltip content={t.openUrl}>
                    <a
                      href={problem.url}
                      target="_blank"
                      rel="noreferrer"
                      aria-label={t.openUrl}
                      className={buttonVariants({
                        variant: "secondary",
                        size: "icon",
                      })}
                    >
                      <Icon icon={ExternalLink} size={14} />
                    </a>
                  </Tooltip>
                ) : null}

                {problem.workspace_path ? (
                  <Tooltip content={t.openFolder}>
                    <Button
                      size="icon"
                      variant="secondary"
                      onClick={() => void handleOpenFolder(problem.id)}
                      aria-label={t.openFolder}
                    >
                      <Icon icon={FolderOpen} size={14} />
                    </Button>
                  </Tooltip>
                ) : null}

                <Tooltip content={t.viewDetails}>
                  <Button
                    size="icon"
                    variant="ghost"
                    onClick={() =>
                      onNavigate({ page: "problem", id: problem.id })
                    }
                    aria-label={t.viewDetails}
                  >
                    <Icon icon={ChevronRight} size={15} />
                  </Button>
                </Tooltip>
              </div>
            </div>
          ))}
        </Card>
      )}
    </div>
  );
}

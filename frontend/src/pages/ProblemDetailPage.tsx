import React, { useCallback, useEffect, useState } from "react";
import {
  AlertCircle,
  ArrowLeft,
  BookOpen,
  Check,
  Copy,
  ExternalLink,
  FileCode2,
  FilePlus2,
  FolderOpen,
  Pencil,
  Trash2,
} from "lucide-react";
import { api } from "../services/api";
import { Icon } from "../components/Icon";
import {
  Badge,
  Button,
  Card,
  Checkbox,
  Dialog,
  DifficultyBadge,
  ErrorBanner,
  Input,
  LoadingState,
  SelectField,
  StatusBadge,
  Switch,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Textarea,
  Tooltip,
  buttonVariants,
} from "../components/ui";
import { ProblemFormModal } from "../components/ProblemFormModal";
import type { Route } from "../components/Layout";
import type { TranslationDict } from "../i18n/translations";
import type {
  AppSettings,
  Language,
  ProblemDetail,
  ProblemStatus,
  Template,
  UpdateProblemInput,
} from "../types/api";
import { useToast } from "../hooks/useToast";

interface ProblemDetailPageProps {
  problemId: number;
  settings: AppSettings;
  languages: Language[];
  templates: Template[];
  t: TranslationDict;
  onNavigate: (r: Route) => void;
  onChanged: () => void;
}

export function ProblemDetailPage({
  problemId,
  settings,
  languages,
  templates,
  t,
  onNavigate,
  onChanged,
}: ProblemDetailPageProps) {
  const { pushToast } = useToast();
  const [detail, setDetail] = useState<ProblemDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [editOpen, setEditOpen] = useState(false);
  const [deleteOpen, setDeleteOpen] = useState(false);
  const [deleteFiles, setDeleteFiles] = useState(false);
  const [deleting, setDeleting] = useState(false);

  // Quick inline notes editor
  const [editingNotes, setEditingNotes] = useState(false);
  const [notesDraft, setNotesDraft] = useState("");
  const [savingNotes, setSavingNotes] = useState(false);

  const [copiedPath, setCopiedPath] = useState(false);

  const [addFileOpen, setAddFileOpen] = useState(false);
  const [fileLang, setFileLang] = useState(settings.default_language || "cpp");
  const [fileName, setFileName] = useState("");
  const [fileTemplate, setFileTemplate] = useState<string>(
    settings.default_template_mode || "starter",
  );
  const [fileSubdir, setFileSubdir] = useState(
    settings.file_layout !== "flat",
  );
  const [addingFile, setAddingFile] = useState(false);
  const [addFileError, setAddFileError] = useState<string | null>(null);

  const [readmeConfirmOpen, setReadmeConfirmOpen] = useState(false);

  const load = useCallback(
    async (recordOpen: boolean) => {
      setLoading(true);
      setError(null);
      try {
        const res = await api.getProblem(problemId, recordOpen);
        setDetail(res);
        setNotesDraft(res.problem.notes);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to load problem");
      } finally {
        setLoading(false);
      }
    },
    [problemId],
  );

  useEffect(() => {
    void load(true);
  }, [load]);

  const handleStatusChange = async (status: ProblemStatus) => {
    if (!detail) return;
    try {
      const updated = await api.updateProblem(detail.problem.id, { status });
      setDetail({ ...detail, problem: updated });
      pushToast(t.toastProblemUpdated, "success");
      onChanged();
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to update status",
        "error",
      );
    }
  };

  const handleEditSave = async (id: number, input: UpdateProblemInput) => {
    const updated = await api.updateProblem(id, input);
    setDetail((prev) => (prev ? { ...prev, problem: updated } : prev));
    setNotesDraft(updated.notes);
    pushToast(t.toastProblemUpdated, "success");
    onChanged();
  };

  const handleSaveInlineNotes = async () => {
    if (!detail) return;
    setSavingNotes(true);
    try {
      const updated = await api.updateProblem(detail.problem.id, {
        notes: notesDraft,
      });
      setDetail((prev) => (prev ? { ...prev, problem: updated } : prev));
      setEditingNotes(false);
      pushToast(t.toastProblemUpdated, "success");
      onChanged();
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to save notes",
        "error",
      );
    } finally {
      setSavingNotes(false);
    }
  };

  const handleDelete = async () => {
    if (!detail) return;
    setDeleting(true);
    try {
      await api.deleteProblem(detail.problem.id, deleteFiles);
      pushToast(t.toastProblemDeleted, "success");
      onChanged();
      onNavigate({ page: "problems" });
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to delete problem",
        "error",
      );
    } finally {
      setDeleting(false);
    }
  };

  const handleOpenFolder = async () => {
    if (!detail) return;
    try {
      await api.openProblemFolder(detail.problem.id);
      pushToast(t.toastFolderOpened, "info");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to open folder",
        "error",
      );
    }
  };

  const handleCopyPath = async (path: string) => {
    try {
      await navigator.clipboard.writeText(path);
      setCopiedPath(true);
      window.setTimeout(() => setCopiedPath(false), 1800);
    } catch {
      // ignore clipboard errors
    }
  };

  const handleAddFile = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!detail) return;
    setAddingFile(true);
    setAddFileError(null);
    try {
      await api.addProblemFile(detail.problem.id, {
        language_id: fileLang,
        filename: fileName.trim() || undefined,
        template_id: fileTemplate,
        subdir: fileSubdir,
      });
      setAddFileOpen(false);
      setFileName("");
      pushToast(t.toastFileAdded, "success");
      await load(false);
      onChanged();
    } catch (err) {
      setAddFileError(
        err instanceof Error ? err.message : "Failed to add file",
      );
    } finally {
      setAddingFile(false);
    }
  };

  const handleGenerateReadme = async (overwrite: boolean) => {
    if (!detail) return;
    try {
      await api.generateReadme(detail.problem.id, { overwrite });
      setReadmeConfirmOpen(false);
      pushToast(t.toastReadmeGenerated, "success");
      await load(false);
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to write README.md",
        "error",
      );
    }
  };

  if (loading && !detail) return <LoadingState label={t.loading} />;
  if (error && !detail) {
    return (
      <div className="space-y-4">
        <Button
          variant="ghost"
          size="sm"
          onClick={() => onNavigate({ page: "problems" })}
        >
          <Icon icon={ArrowLeft} size={15} />
          {t.backToProblems}
        </Button>
        <ErrorBanner
          message={error}
          onRetry={() => void load(false)}
          retryLabel={t.retry}
        />
      </div>
    );
  }
  if (!detail) return null;

  const { problem, files, workspace_exists } = detail;
  const hasReadme = files.some((f) => f.is_readme);

  const customLangTemplates = templates.filter(
    (tpl) =>
      tpl.kind === "source" &&
      tpl.language_id === fileLang &&
      !tpl.is_builtin,
  );

  return (
    <div className="space-y-6">
      <div>
        <Button
          variant="ghost"
          size="sm"
          onClick={() => onNavigate({ page: "problems" })}
          className="-ml-2"
        >
          <Icon icon={ArrowLeft} size={14} />
          {t.backToProblems}
        </Button>
      </div>

      {/* Header */}
      <Card className="p-5 sm:p-6">
        <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
          <div className="min-w-0 flex-1 space-y-2.5">
            <div className="flex flex-wrap items-center gap-2.5">
              <h1 className="text-xl font-semibold tracking-tight text-[var(--text-primary)]">
                {problem.title}
              </h1>
              {problem.identifier ? (
                <span className="rounded-full bg-[var(--bg-subtle)] px-2.5 py-0.5 font-mono text-xs text-[var(--text-muted)]">
                  {problem.identifier}
                </span>
              ) : null}
            </div>

            <div className="flex flex-wrap items-center gap-1.5">
              <StatusBadge status={problem.status} t={t} />
              <DifficultyBadge difficulty={problem.difficulty} t={t} />
              {problem.platform ? <Badge>{problem.platform}</Badge> : null}
              {problem.languages.map((lang) => (
                <span
                  key={lang}
                  className="rounded-full bg-[var(--bg-subtle)] px-2.5 py-0.5 font-mono text-[11px] text-[var(--text-secondary)]"
                >
                  {lang}
                </span>
              ))}
              {problem.tags.map((tag) => (
                <span
                  key={tag}
                  className="rounded-full border border-[var(--border-subtle)] px-2.5 py-0.5 text-[11px] text-[var(--text-secondary)]"
                >
                  #{tag}
                </span>
              ))}
            </div>
          </div>

          <div className="flex shrink-0 flex-wrap items-center gap-2">
            <div className="w-36">
              <SelectField
                aria-label={t.status}
                value={problem.status}
                onValueChange={(v) =>
                  void handleStatusChange(v as ProblemStatus)
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
              <a
                href={problem.url}
                target="_blank"
                rel="noreferrer"
                className={buttonVariants({ variant: "secondary", size: "md" })}
              >
                <Icon icon={ExternalLink} size={14} />
                {t.openUrl}
              </a>
            ) : null}

            {problem.workspace_path ? (
              <Button variant="secondary" onClick={() => void handleOpenFolder()}>
                <Icon icon={FolderOpen} size={14} />
                {t.openFolder}
              </Button>
            ) : null}

            <Button variant="secondary" onClick={() => setEditOpen(true)}>
              <Icon icon={Pencil} size={14} />
              {t.edit}
            </Button>

            <Tooltip content={t.deleteProblem}>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => {
                  setDeleteFiles(false);
                  setDeleteOpen(true);
                }}
                aria-label={t.deleteProblem}
                style={{ color: "var(--danger)" }}
              >
                <Icon icon={Trash2} size={15} />
              </Button>
            </Tooltip>
          </div>
        </div>

        <div className="mt-5 grid grid-cols-1 gap-3 border-t border-[var(--border-subtle)] pt-4 text-xs sm:grid-cols-3">
          <div>
            <span className="text-[var(--text-muted)]">{t.createdAt}: </span>
            <span className="font-mono text-[var(--text-secondary)]">
              {problem.created_at.slice(0, 10)}
            </span>
          </div>
          <div>
            <span className="text-[var(--text-muted)]">{t.updatedAt}: </span>
            <span className="font-mono text-[var(--text-secondary)]">
              {problem.updated_at.slice(0, 10)}
            </span>
          </div>
          <div>
            <span className="text-[var(--text-muted)]">{t.lastOpenedAt}: </span>
            <span className="font-mono text-[var(--text-secondary)]">
              {problem.last_opened_at
                ? problem.last_opened_at.slice(0, 10)
                : "—"}
            </span>
          </div>
        </div>
      </Card>

      <div className="grid grid-cols-1 gap-5 lg:grid-cols-3">
        {/* Left: Problem Statement & Notes */}
        <div className="lg:col-span-2">
          <Tabs defaultValue="overview" className="space-y-4">
            <TabsList>
              <TabsTrigger value="overview">{t.overview}</TabsTrigger>
              <TabsTrigger value="description">{t.description}</TabsTrigger>
              <TabsTrigger value="notes">{t.notes}</TabsTrigger>
            </TabsList>

            <TabsContent value="overview" className="space-y-5">
              <Card className="p-5">
                <div className="flex items-center justify-between">
                  <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                    {t.description}
                  </h2>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => setEditOpen(true)}
                  >
                    <Icon icon={Pencil} size={13} />
                    {t.edit}
                  </Button>
                </div>
                <p className="mt-2.5 whitespace-pre-wrap text-[13px] leading-relaxed text-[var(--text-secondary)]">
                  {problem.description || t.noDescription}
                </p>
              </Card>

              <Card className="p-5">
                <div className="flex items-center justify-between">
                  <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                    {t.notes}
                  </h2>
                  {!editingNotes ? (
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => {
                        setNotesDraft(problem.notes);
                        setEditingNotes(true);
                      }}
                    >
                      <Icon icon={Pencil} size={13} />
                      {t.edit}
                    </Button>
                  ) : null}
                </div>
                {editingNotes ? (
                  <div className="mt-3 space-y-3">
                    <Textarea
                      rows={6}
                      autoFocus
                      value={notesDraft}
                      onChange={(e) => setNotesDraft(e.target.value)}
                      placeholder={t.notesPlaceholder}
                    />
                    <div className="flex justify-end gap-2">
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={() => {
                          setNotesDraft(problem.notes);
                          setEditingNotes(false);
                        }}
                      >
                        {t.cancel}
                      </Button>
                      <Button
                        size="sm"
                        variant="primary"
                        disabled={savingNotes}
                        onClick={() => void handleSaveInlineNotes()}
                      >
                        {savingNotes ? t.saving : t.save}
                      </Button>
                    </div>
                  </div>
                ) : (
                  <p className="mt-2.5 whitespace-pre-wrap text-[13px] leading-relaxed text-[var(--text-secondary)]">
                    {problem.notes || t.noNotes}
                  </p>
                )}
              </Card>
            </TabsContent>

            <TabsContent value="description">
              <Card className="p-5">
                <div className="flex items-center justify-between">
                  <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                    {t.description}
                  </h2>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => setEditOpen(true)}
                  >
                    <Icon icon={Pencil} size={13} />
                    {t.edit}
                  </Button>
                </div>
                <p className="mt-3 whitespace-pre-wrap text-[13px] leading-relaxed text-[var(--text-secondary)]">
                  {problem.description || t.noDescription}
                </p>
              </Card>
            </TabsContent>

            <TabsContent value="notes">
              <Card className="p-5">
                <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                  {t.notes}
                </h2>
                <div className="mt-3 space-y-3">
                  <Textarea
                    rows={10}
                    value={notesDraft}
                    onChange={(e) => setNotesDraft(e.target.value)}
                    placeholder={t.notesPlaceholder}
                  />
                  <div className="flex justify-end">
                    <Button
                      size="sm"
                      variant="primary"
                      disabled={savingNotes || notesDraft === problem.notes}
                      onClick={() => void handleSaveInlineNotes()}
                    >
                      {savingNotes ? t.saving : t.save}
                    </Button>
                  </div>
                </div>
              </Card>
            </TabsContent>
          </Tabs>
        </div>

        {/* Right: Workspace Files */}
        <div className="space-y-4">
          <Card className="p-5">
            <div className="flex items-center justify-between gap-2">
              <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                {t.sourceFiles}
              </h2>
              {workspace_exists ? (
                <Button
                  size="sm"
                  variant="secondary"
                  onClick={() => {
                    setAddFileError(null);
                    setAddFileOpen(true);
                  }}
                >
                  <Icon icon={FilePlus2} size={14} />
                  {t.addFile}
                </Button>
              ) : null}
            </div>

            {problem.workspace_path ? (
              <div className="mt-3 flex items-center gap-1.5 rounded-full border border-[var(--border-subtle)] bg-[var(--bg-canvas)] px-3 py-1.5">
                <span
                  className="min-w-0 flex-1 truncate font-mono text-[11px] text-[var(--text-muted)]"
                  title={problem.workspace_path}
                >
                  {problem.workspace_path}
                </span>
                <button
                  type="button"
                  onClick={() => void handleCopyPath(problem.workspace_path)}
                  aria-label="Copy path"
                  className="shrink-0 rounded-full p-1 text-[var(--text-muted)] hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)] cursor-pointer"
                >
                  <Icon icon={copiedPath ? Check : Copy} size={12} />
                </button>
              </div>
            ) : null}

            {!workspace_exists ? (
              <div
                className="mt-3 flex items-start gap-2 rounded-xl border p-3 text-xs"
                style={{
                  backgroundColor: "var(--warning-soft)",
                  borderColor: "var(--warning)",
                  color: "var(--warning)",
                }}
              >
                <Icon icon={AlertCircle} size={15} className="mt-0.5 shrink-0" />
                <span>{t.folderMissingWarning}</span>
              </div>
            ) : (
              <>
                <div className="mt-3 divide-y divide-[var(--border-subtle)] overflow-hidden rounded-xl border border-[var(--border-subtle)]">
                  {files.length === 0 ? (
                    <div className="px-3 py-5 text-center text-xs text-[var(--text-muted)]">
                      —
                    </div>
                  ) : (
                    files.map((file) => (
                      <div
                        key={file.relative_path}
                        className="flex items-center justify-between gap-2 px-3 py-2 text-xs"
                      >
                        <div className="flex min-w-0 items-center gap-2">
                          <Icon
                            icon={file.is_readme ? BookOpen : FileCode2}
                            size={14}
                            className="shrink-0 text-[var(--text-muted)]"
                          />
                          <span className="truncate font-mono text-[var(--text-primary)]">
                            {file.relative_path}
                          </span>
                        </div>
                        <span className="shrink-0 font-mono text-[11px] text-[var(--text-muted)]">
                          {file.size_bytes} B
                        </span>
                      </div>
                    ))
                  )}
                </div>

                <div className="mt-3">
                  <Button
                    size="sm"
                    variant="secondary"
                    className="w-full"
                    onClick={() => {
                      if (hasReadme) {
                        setReadmeConfirmOpen(true);
                      } else {
                        void handleGenerateReadme(false);
                      }
                    }}
                  >
                    <Icon icon={BookOpen} size={14} />
                    {hasReadme ? t.regenerateReadme : t.generateReadme}
                  </Button>
                </div>
              </>
            )}
          </Card>
        </div>
      </div>

      {/* Edit Modal */}
      <ProblemFormModal
        open={editOpen}
        onClose={() => setEditOpen(false)}
        mode="edit"
        initialProblem={problem}
        settings={settings}
        languages={languages}
        templates={templates}
        t={t}
        onCreate={async () => {}}
        onUpdate={handleEditSave}
      />

      {/* Add Language File Modal */}
      <Dialog
        open={addFileOpen}
        onClose={() => setAddFileOpen(false)}
        title={t.addFileTitle}
        maxWidth="max-w-md"
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={() => setAddFileOpen(false)}>
              {t.cancel}
            </Button>
            <Button
              type="submit"
              form="ducky-add-file-form"
              variant="primary"
              disabled={addingFile}
            >
              {addingFile ? t.creating : t.create}
            </Button>
          </div>
        }
      >
        <form
          id="ducky-add-file-form"
          onSubmit={handleAddFile}
          className="space-y-4"
        >
          {addFileError ? (
            <div
              className="rounded-xl border px-3 py-2 text-xs"
              style={{
                backgroundColor: "var(--danger-soft)",
                borderColor: "var(--danger)",
                color: "var(--danger)",
              }}
            >
              {addFileError}
            </div>
          ) : null}

          <div>
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.language}
            </label>
            <SelectField
              value={fileLang}
              onValueChange={setFileLang}
              options={languages
                .filter((l) => l.enabled)
                .map((l) => ({
                  value: l.id,
                  label: `${l.name} (${l.default_filename})`,
                }))}
            />
          </div>

          <div>
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.filenameOptional}
            </label>
            <Input
              value={fileName}
              onChange={(e) => setFileName(e.target.value)}
              placeholder={t.filenamePlaceholder}
              className="font-mono text-xs"
            />
          </div>

          <div>
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.template}
            </label>
            <SelectField
              value={fileTemplate}
              onValueChange={setFileTemplate}
              options={[
                { value: "starter", label: t.templateStarter },
                { value: "empty", label: t.templateEmpty },
                ...customLangTemplates.map((tpl) => ({
                  value: tpl.id,
                  label: tpl.name,
                })),
              ]}
            />
          </div>

          <label className="flex items-center justify-between gap-3 pt-1 text-xs text-[var(--text-secondary)] cursor-pointer">
            <span>{t.placeInSubdir}</span>
            <Switch checked={fileSubdir} onCheckedChange={setFileSubdir} />
          </label>
        </form>
      </Dialog>

      {/* Overwrite README confirmation */}
      <Dialog
        open={readmeConfirmOpen}
        onClose={() => setReadmeConfirmOpen(false)}
        title={t.readmeOverwriteConfirmTitle}
        maxWidth="max-w-md"
        footer={
          <div className="flex justify-end gap-2">
            <Button
              variant="ghost"
              onClick={() => setReadmeConfirmOpen(false)}
            >
              {t.cancel}
            </Button>
            <Button
              variant="primary"
              onClick={() => void handleGenerateReadme(true)}
            >
              {t.confirm}
            </Button>
          </div>
        }
      >
        <p className="text-[13px] leading-relaxed text-[var(--text-secondary)]">
          {t.readmeOverwriteConfirmBody}
        </p>
      </Dialog>

      {/* Delete confirmation */}
      <Dialog
        open={deleteOpen}
        onClose={() => setDeleteOpen(false)}
        title={t.deleteConfirmTitle}
        maxWidth="max-w-md"
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={() => setDeleteOpen(false)}>
              {t.cancel}
            </Button>
            <Button
              variant="danger"
              disabled={deleting}
              onClick={() => void handleDelete()}
            >
              {t.delete}
            </Button>
          </div>
        }
      >
        <div className="space-y-4">
          <p className="text-[13px] leading-relaxed text-[var(--text-secondary)]">
            {t.deleteConfirmBody}
          </p>
          {workspace_exists ? (
            <label className="flex items-start gap-2.5 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-canvas)] p-3 text-xs text-[var(--text-secondary)] cursor-pointer">
              <Checkbox
                checked={deleteFiles}
                onCheckedChange={setDeleteFiles}
                className="mt-0.5"
              />
              <span>{t.deleteAlsoFiles}</span>
            </label>
          ) : null}
        </div>
      </Dialog>
    </div>
  );
}

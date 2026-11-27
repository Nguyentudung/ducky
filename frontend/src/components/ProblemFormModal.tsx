import React, { useEffect, useState } from "react";
import { ChevronDown, ChevronUp } from "lucide-react";
import { Icon } from "./Icon";
import {
  Button,
  Dialog,
  Input,
  SelectField,
  Switch,
  Textarea,
  cn,
} from "./ui";
import type {
  AppSettings,
  CreateProblemInput,
  FileLayout,
  Language,
  Problem,
  ProblemDifficulty,
  ProblemStatus,
  Template,
  UpdateProblemInput,
} from "../types/api";
import type { TranslationDict } from "../i18n/translations";

interface ProblemFormModalProps {
  open: boolean;
  onClose: () => void;
  mode: "create" | "edit";
  initialProblem?: Problem;
  settings: AppSettings;
  languages: Language[];
  templates: Template[];
  t: TranslationDict;
  onCreate: (input: CreateProblemInput) => Promise<void>;
  onUpdate: (id: number, input: UpdateProblemInput) => Promise<void>;
}

const PLATFORM_PRESETS = ["Codeforces", "LeetCode", "AtCoder", "CSES"];

export function ProblemFormModal({
  open,
  onClose,
  mode,
  initialProblem,
  settings,
  languages,
  templates,
  t,
  onCreate,
  onUpdate,
}: ProblemFormModalProps) {
  const enabledLanguages = languages.filter((l) => l.enabled);

  const [title, setTitle] = useState("");
  const [platform, setPlatform] = useState("");
  const [selectedLangs, setSelectedLangs] = useState<string[]>([]);
  const [status, setStatus] = useState<ProblemStatus>("not_started");
  const [difficulty, setDifficulty] = useState<ProblemDifficulty>("medium");
  const [identifier, setIdentifier] = useState("");
  const [url, setUrl] = useState("");
  const [tagsRaw, setTagsRaw] = useState("");
  const [description, setDescription] = useState("");
  const [notes, setNotes] = useState("");
  const [folderName, setFolderName] = useState("");
  const [groupByPlatform, setGroupByPlatform] = useState(true);
  const [fileLayout, setFileLayout] = useState<FileLayout>("auto");
  const [templateMode, setTemplateMode] = useState<string>("starter");
  const [generateReadme, setGenerateReadme] = useState(true);
  const [customFilenames, setCustomFilenames] = useState<Record<string, string>>(
    {},
  );
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!open) return;
    setError(null);
    if (mode === "edit" && initialProblem) {
      setTitle(initialProblem.title);
      setPlatform(initialProblem.platform);
      setSelectedLangs(initialProblem.languages);
      setStatus(initialProblem.status);
      setDifficulty(initialProblem.difficulty);
      setIdentifier(initialProblem.identifier);
      setUrl(initialProblem.url);
      setTagsRaw(initialProblem.tags.join(", "));
      setDescription(initialProblem.description);
      setNotes(initialProblem.notes);
      setShowAdvanced(true);
    } else {
      setTitle("");
      setPlatform("");
      const defLang =
        enabledLanguages.find((l) => l.id === settings.default_language)?.id ??
        enabledLanguages[0]?.id ??
        "cpp";
      setSelectedLangs([defLang]);
      setStatus(settings.default_status);
      setDifficulty(settings.default_difficulty);
      setIdentifier("");
      setUrl("");
      setTagsRaw("");
      setDescription("");
      setNotes("");
      setFolderName("");
      setGroupByPlatform(settings.group_by_platform);
      setFileLayout(settings.file_layout);
      setTemplateMode(settings.default_template_mode);
      setGenerateReadme(settings.generate_readme);
      setCustomFilenames({});
      setShowAdvanced(false);
    }
  }, [open, mode, initialProblem, settings]);

  const toggleLanguage = (id: string) => {
    setSelectedLangs((prev) => {
      if (prev.includes(id)) {
        return prev.length > 1 ? prev.filter((x) => x !== id) : prev;
      }
      return [...prev, id];
    });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setSubmitting(true);

    const tags = tagsRaw
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);

    try {
      if (mode === "create") {
        await onCreate({
          title,
          identifier,
          platform,
          url,
          difficulty,
          status,
          tags,
          languages: selectedLangs,
          description,
          notes,
          folder_name: folderName.trim() || undefined,
          group_by_platform: groupByPlatform,
          file_layout: fileLayout,
          template_mode: templateMode,
          custom_filenames: customFilenames,
          generate_readme: generateReadme,
        });
      } else if (initialProblem) {
        await onUpdate(initialProblem.id, {
          title,
          identifier,
          platform,
          url,
          difficulty,
          status,
          tags,
          languages: selectedLangs,
          description,
          notes,
        });
      }
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to save problem");
    } finally {
      setSubmitting(false);
    }
  };

  const sourceTemplates = templates.filter(
    (tpl) =>
      tpl.kind === "source" &&
      !tpl.is_builtin &&
      tpl.language_id &&
      selectedLangs.includes(tpl.language_id),
  );

  return (
    <Dialog
      open={open}
      onClose={onClose}
      title={mode === "create" ? t.createProblemTitle : t.editProblemTitle}
      maxWidth="max-w-xl"
      footer={
        <div className="flex items-center justify-end gap-2">
          <Button variant="ghost" onClick={onClose}>
            {t.cancel}
          </Button>
          <Button
            type="submit"
            form="ducky-problem-form"
            variant="primary"
            disabled={submitting}
          >
            {submitting
              ? mode === "create"
                ? t.creating
                : t.saving
              : mode === "create"
                ? t.create
                : t.save}
          </Button>
        </div>
      }
    >
      <form id="ducky-problem-form" onSubmit={handleSubmit} className="space-y-4">
        {error ? (
          <div
            className="rounded-xl border px-3.5 py-2.5 text-[13px]"
            style={{
              backgroundColor: "var(--danger-soft)",
              borderColor: "var(--danger)",
              color: "var(--danger)",
            }}
          >
            {error}
          </div>
        ) : null}

        {/* Essential Fields */}
        <div>
          <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
            {t.problemTitle} <span className="text-[var(--danger)]">*</span>
          </label>
          <Input
            required
            autoFocus
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder={t.problemTitlePlaceholder}
          />
        </div>

        <div className="grid grid-cols-1 gap-3 sm:grid-cols-12">
          <div className="sm:col-span-5">
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.platform}
            </label>
            <Input
              value={platform}
              onChange={(e) => setPlatform(e.target.value)}
              placeholder={t.platformPlaceholder}
              list="ducky-platform-presets"
            />
            <datalist id="ducky-platform-presets">
              {PLATFORM_PRESETS.map((p) => (
                <option key={p} value={p} />
              ))}
            </datalist>
          </div>

          <div className="sm:col-span-3">
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.difficulty}
            </label>
            <SelectField
              value={difficulty}
              onValueChange={(v) => setDifficulty(v as ProblemDifficulty)}
              options={(
                ["easy", "medium", "hard", "custom"] as ProblemDifficulty[]
              ).map((d) => ({
                value: d,
                label: t.difficultyLabels[d],
              }))}
            />
          </div>

          <div className="sm:col-span-4">
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.status}
            </label>
            <SelectField
              value={status}
              onValueChange={(v) => setStatus(v as ProblemStatus)}
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
        </div>

        {/* Platform quick-pick pill chips */}
        <div className="-mt-1 flex flex-wrap items-center gap-1.5">
          {PLATFORM_PRESETS.map((preset) => (
            <button
              key={preset}
              type="button"
              onClick={() => setPlatform(platform === preset ? "" : preset)}
              className={cn(
                "rounded-full px-3 py-1 text-[11px] font-medium transition-colors cursor-pointer",
                platform === preset
                  ? "bg-[var(--accent-soft)] text-[var(--accent-text)]"
                  : "bg-[var(--bg-subtle)] text-[var(--text-muted)] hover:text-[var(--text-primary)]",
              )}
            >
              {preset}
            </button>
          ))}
        </div>

        <div>
          <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
            {t.problemUrl}
          </label>
          <Input
            type="url"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            placeholder="https://..."
          />
        </div>

        <div>
          <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
            {t.languages}
          </label>
          <div className="flex flex-wrap gap-1.5">
            {(enabledLanguages.length > 0 ? enabledLanguages : languages).map(
              (lang) => {
                const selected = selectedLangs.includes(lang.id);
                return (
                  <button
                    key={lang.id}
                    type="button"
                    onClick={() => toggleLanguage(lang.id)}
                    className={cn(
                      "rounded-full border px-3.5 py-1.5 text-xs font-medium transition-all cursor-pointer",
                      selected
                        ? "border-[var(--accent)] bg-[var(--accent-soft)] text-[var(--accent-text)]"
                        : "border-[var(--border-subtle)] bg-[var(--bg-surface)] text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)]",
                    )}
                  >
                    {lang.name}
                    <span className="ml-1.5 font-mono text-[11px] opacity-65">
                      .{lang.extension}
                    </span>
                  </button>
                );
              },
            )}
          </div>
        </div>

        {/* Progressive Disclosure */}
        <div className="border-t border-[var(--border-subtle)] pt-3">
          <button
            type="button"
            onClick={() => setShowAdvanced((v) => !v)}
            className="flex w-full items-center justify-between rounded-full px-2 py-1.5 text-left text-xs font-medium text-[var(--text-secondary)] transition-colors hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)] cursor-pointer"
          >
            <span>{t.advancedOptions}</span>
            <Icon icon={showAdvanced ? ChevronUp : ChevronDown} size={15} />
          </button>

          {showAdvanced ? (
            <div className="mt-3 space-y-3.5 rounded-2xl border border-[var(--border-subtle)] bg-[var(--bg-canvas)]/70 p-4">
              <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
                <div>
                  <label className="mb-1 block text-xs font-medium text-[var(--text-secondary)]">
                    {t.identifier}
                  </label>
                  <Input
                    value={identifier}
                    onChange={(e) => setIdentifier(e.target.value)}
                    placeholder={t.identifierPlaceholder}
                    className="font-mono text-xs"
                  />
                </div>
                <div>
                  <label className="mb-1 block text-xs font-medium text-[var(--text-secondary)]">
                    {t.tags}
                  </label>
                  <Input
                    value={tagsRaw}
                    onChange={(e) => setTagsRaw(e.target.value)}
                    placeholder={t.tagsPlaceholder}
                  />
                </div>
              </div>

              <div>
                <label className="mb-1 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.description}
                </label>
                <Textarea
                  rows={3}
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder={t.descriptionPlaceholder}
                />
              </div>

              <div>
                <label className="mb-1 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.notes}
                </label>
                <Textarea
                  rows={3}
                  value={notes}
                  onChange={(e) => setNotes(e.target.value)}
                  placeholder={t.notesPlaceholder}
                />
              </div>

              {mode === "create" ? (
                <div className="space-y-3.5 border-t border-[var(--border-subtle)] pt-3.5">
                  <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
                    <div>
                      <label className="mb-1 block text-xs font-medium text-[var(--text-secondary)]">
                        {t.customFolderName}
                      </label>
                      <Input
                        value={folderName}
                        onChange={(e) => setFolderName(e.target.value)}
                        placeholder={t.customFolderPlaceholder}
                        className="font-mono text-xs"
                      />
                    </div>
                    <div>
                      <label className="mb-1 block text-xs font-medium text-[var(--text-secondary)]">
                        {t.sourceTemplate}
                      </label>
                      <SelectField
                        value={templateMode}
                        onValueChange={setTemplateMode}
                        options={[
                          { value: "starter", label: t.templateStarter },
                          { value: "empty", label: t.templateEmpty },
                          ...sourceTemplates.map((tpl) => ({
                            value: tpl.id,
                            label: `${tpl.name} (${tpl.language_id})`,
                          })),
                        ]}
                      />
                    </div>
                  </div>

                  <div>
                    <label className="mb-1 block text-xs font-medium text-[var(--text-secondary)]">
                      {t.fileLayout}
                    </label>
                    <SelectField
                      value={fileLayout}
                      onValueChange={(v) => setFileLayout(v as FileLayout)}
                      options={[
                        { value: "auto", label: t.layoutAuto },
                        { value: "subdirs", label: t.layoutSubdirs },
                        { value: "flat", label: t.layoutFlat },
                      ]}
                    />
                  </div>

                  {selectedLangs.length > 0 ? (
                    <div>
                      <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                        {t.customFilenames}
                      </label>
                      <div className="grid grid-cols-1 gap-2 sm:grid-cols-2">
                        {selectedLangs.map((langId) => {
                          const lang = languages.find((l) => l.id === langId);
                          return (
                            <div
                              key={langId}
                              className="flex items-center gap-2"
                            >
                              <span className="w-14 shrink-0 font-mono text-xs text-[var(--text-muted)]">
                                {lang?.name ?? langId}
                              </span>
                              <Input
                                value={customFilenames[langId] ?? ""}
                                onChange={(e) =>
                                  setCustomFilenames((prev) => ({
                                    ...prev,
                                    [langId]: e.target.value,
                                  }))
                                }
                                placeholder={lang?.default_filename ?? "main"}
                                className="h-8 font-mono text-xs"
                              />
                            </div>
                          );
                        })}
                      </div>
                    </div>
                  ) : null}

                  <div className="flex flex-wrap gap-5 pt-1">
                    <label className="flex items-center gap-2.5 text-xs text-[var(--text-secondary)] cursor-pointer">
                      <Switch
                        checked={groupByPlatform}
                        onCheckedChange={setGroupByPlatform}
                      />
                      <span>{t.groupByPlatform}</span>
                    </label>
                    <label className="flex items-center gap-2.5 text-xs text-[var(--text-secondary)] cursor-pointer">
                      <Switch
                        checked={generateReadme}
                        onCheckedChange={setGenerateReadme}
                      />
                      <span>{t.generateReadmeOnCreate}</span>
                    </label>
                  </div>
                </div>
              ) : null}
            </div>
          ) : null}
        </div>
      </form>
    </Dialog>
  );
}

import React, { useEffect, useState } from "react";
import {
  Code2,
  FileCode2,
  Folder,
  FolderOpen,
  Monitor,
  Moon,
  Palette,
  Pencil,
  Plus,
  Sliders,
  Sun,
  Trash2,
  type LucideIcon,
} from "lucide-react";
import { api } from "../services/api";
import { Icon } from "../components/Icon";
import {
  Badge,
  Button,
  Card,
  Checkbox,
  Dialog,
  Input,
  SelectField,
  Switch,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Textarea,
  cn,
} from "../components/ui";
import type { TranslationDict } from "../i18n/translations";
import type {
  AppSettings,
  FileLayout,
  Language,
  LanguagePreference,
  ProblemDifficulty,
  ProblemStatus,
  Template,
  TemplateMode,
  ThemePreference,
  WorkspaceStatus,
} from "../types/api";
import { useToast } from "../hooks/useToast";

type SettingsTab =
  | "general"
  | "appearance"
  | "workspace"
  | "languages"
  | "templates";

interface SettingsPageProps {
  initialTab?: string;
  settings: AppSettings;
  languages: Language[];
  templates: Template[];
  t: TranslationDict;
  onSettingsUpdated: (next: AppSettings) => void;
  onLanguagesChanged: () => Promise<void>;
  onTemplatesChanged: () => Promise<void>;
  onOpenWorkspace: () => void;
}

export function SettingsPage({
  initialTab,
  settings,
  languages,
  templates,
  t,
  onSettingsUpdated,
  onLanguagesChanged,
  onTemplatesChanged,
  onOpenWorkspace,
}: SettingsPageProps) {
  const { pushToast } = useToast();
  const [tab, setTab] = useState<SettingsTab>(() => {
    if (
      initialTab &&
      ["general", "appearance", "workspace", "languages", "templates"].includes(
        initialTab,
      )
    ) {
      return initialTab as SettingsTab;
    }
    return "general";
  });

  useEffect(() => {
    if (
      initialTab &&
      ["general", "appearance", "workspace", "languages", "templates"].includes(
        initialTab,
      )
    ) {
      setTab(initialTab as SettingsTab);
    }
  }, [initialTab]);

  // Workspace form state
  const [wsInput, setWsInput] = useState(settings.workspace_path);
  const [wsValidation, setWsValidation] = useState<WorkspaceStatus | null>(
    null,
  );
  const [wsSaving, setWsSaving] = useState(false);

  // Language modal state
  const [langModalOpen, setLangModalOpen] = useState(false);
  const [editingLang, setEditingLang] = useState<Language | null>(null);
  const [langId, setLangId] = useState("");
  const [langName, setLangName] = useState("");
  const [langExt, setLangExt] = useState("");
  const [langFile, setLangFile] = useState("");
  const [langFolder, setLangFolder] = useState("");
  const [langError, setLangError] = useState<string | null>(null);

  // Template modal state
  const [tplModalOpen, setTplModalOpen] = useState(false);
  const [editingTpl, setEditingTpl] = useState<Template | null>(null);
  const [tplName, setTplName] = useState("");
  const [tplKind, setTplKind] = useState<"source" | "readme">("source");
  const [tplLangId, setTplLangId] = useState(languages[0]?.id ?? "cpp");
  const [tplContent, setTplContent] = useState("");
  const [tplDefault, setTplDefault] = useState(false);
  const [tplError, setTplError] = useState<string | null>(null);

  useEffect(() => {
    setWsInput(settings.workspace_path);
  }, [settings.workspace_path]);

  const patch = async (partial: Partial<AppSettings>) => {
    try {
      const next = await api.patchSettings(partial);
      onSettingsUpdated(next);
      pushToast(t.toastSettingsSaved, "success");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to save settings",
        "error",
      );
    }
  };

  const handleValidateWorkspace = async () => {
    try {
      const status = await api.validateWorkspace(wsInput);
      setWsValidation(status);
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Validation failed",
        "error",
      );
    }
  };

  const handleSaveWorkspace = async (e: React.FormEvent) => {
    e.preventDefault();
    setWsSaving(true);
    try {
      const next = await api.patchSettings({ workspace_path: wsInput });
      onSettingsUpdated(next);
      const status = await api.getWorkspace();
      setWsValidation(status);
      pushToast(t.toastSettingsSaved, "success");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to save workspace",
        "error",
      );
    } finally {
      setWsSaving(false);
    }
  };

  const openCreateLanguage = () => {
    setEditingLang(null);
    setLangId("");
    setLangName("");
    setLangExt("");
    setLangFile("");
    setLangFolder("");
    setLangError(null);
    setLangModalOpen(true);
  };

  const openEditLanguage = (lang: Language) => {
    setEditingLang(lang);
    setLangId(lang.id);
    setLangName(lang.name);
    setLangExt(lang.extension);
    setLangFile(lang.default_filename);
    setLangFolder(lang.folder_name);
    setLangError(null);
    setLangModalOpen(true);
  };

  const submitLanguage = async (e: React.FormEvent) => {
    e.preventDefault();
    setLangError(null);
    try {
      if (editingLang) {
        await api.updateLanguage(editingLang.id, {
          name: langName,
          extension: langExt,
          default_filename: langFile,
          folder_name: langFolder,
        });
      } else {
        await api.createLanguage({
          id: langId,
          name: langName,
          extension: langExt,
          default_filename: langFile,
          folder_name: langFolder,
        });
      }
      setLangModalOpen(false);
      await onLanguagesChanged();
      pushToast(t.toastLanguageSaved, "success");
    } catch (err) {
      setLangError(err instanceof Error ? err.message : "Failed to save");
    }
  };

  const toggleLanguageEnabled = async (lang: Language) => {
    try {
      await api.updateLanguage(lang.id, { enabled: !lang.enabled });
      await onLanguagesChanged();
      pushToast(t.toastLanguageSaved, "success");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to update language",
        "error",
      );
    }
  };

  const handleDeleteLanguage = async (id: string) => {
    try {
      await api.deleteLanguage(id);
      await onLanguagesChanged();
      pushToast(t.toastLanguageDeleted, "success");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to delete language",
        "error",
      );
    }
  };

  const openCreateTemplate = () => {
    setEditingTpl(null);
    setTplName("");
    setTplKind("source");
    setTplLangId(languages[0]?.id ?? "cpp");
    setTplContent("");
    setTplDefault(false);
    setTplError(null);
    setTplModalOpen(true);
  };

  const openEditTemplate = (tpl: Template) => {
    setEditingTpl(tpl);
    setTplName(tpl.name);
    setTplKind(tpl.kind);
    setTplLangId(tpl.language_id ?? languages[0]?.id ?? "cpp");
    setTplContent(tpl.content);
    setTplDefault(tpl.is_default);
    setTplError(null);
    setTplModalOpen(true);
  };

  const submitTemplate = async (e: React.FormEvent) => {
    e.preventDefault();
    setTplError(null);
    try {
      if (editingTpl) {
        await api.updateTemplate(editingTpl.id, {
          name: tplName,
          content: tplContent,
          is_default: tplDefault,
        });
      } else {
        await api.createTemplate({
          name: tplName,
          kind: tplKind,
          language_id: tplKind === "source" ? tplLangId : null,
          content: tplContent,
          is_default: tplDefault,
        });
      }
      setTplModalOpen(false);
      await onTemplatesChanged();
      pushToast(t.toastTemplateSaved, "success");
    } catch (err) {
      setTplError(err instanceof Error ? err.message : "Failed to save");
    }
  };

  const handleDeleteTemplate = async (id: string) => {
    try {
      await api.deleteTemplate(id);
      await onTemplatesChanged();
      pushToast(t.toastTemplateDeleted, "success");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to delete template",
        "error",
      );
    }
  };

  const tabs: Array<{ id: SettingsTab; label: string; icon: LucideIcon }> = [
    { id: "general", label: t.tabGeneral, icon: Sliders },
    { id: "appearance", label: t.tabAppearance, icon: Palette },
    { id: "workspace", label: t.tabWorkspace, icon: Folder },
    { id: "languages", label: t.tabLanguages, icon: Code2 },
    { id: "templates", label: t.tabTemplates, icon: FileCode2 },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-[var(--text-primary)]">
          {t.settingsTitle}
        </h1>
        <p className="mt-1 text-[13px] text-[var(--text-secondary)]">
          {t.settingsSubtitle}
        </p>
      </div>

      {/* Capsule Segmented Control (shadcn/ui Tabs) */}
      <Tabs
        value={tab}
        onValueChange={(v) => setTab(v as SettingsTab)}
        className="space-y-5"
      >
        <TabsList>
          {tabs.map((item) => (
            <TabsTrigger key={item.id} value={item.id}>
              <Icon icon={item.icon} size={14} />
              {item.label}
            </TabsTrigger>
          ))}
        </TabsList>

        {/* GENERAL */}
        <TabsContent value="general" className="space-y-5">
          <Card className="divide-y divide-[var(--border-subtle)]">
            <div className="flex flex-col gap-3 p-5 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                  {t.appLanguageLabel}
                </h2>
                <p className="mt-0.5 text-xs text-[var(--text-secondary)]">
                  {t.appLanguageDesc}
                </p>
              </div>
              <div className="w-full sm:w-52">
                <SelectField
                  aria-label={t.appLanguageLabel}
                  value={settings.language}
                  onValueChange={(val) =>
                    void patch({ language: val as LanguagePreference })
                  }
                  options={[
                    { value: "system", label: t.langSystem },
                    { value: "en", label: t.langEn },
                    { value: "vi", label: t.langVi },
                  ]}
                />
              </div>
            </div>
          </Card>

          <Card className="p-5">
            <div>
              <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                {t.problemDefaults}
              </h2>
              <p className="mt-0.5 text-xs text-[var(--text-secondary)]">
                {t.problemDefaultsDesc}
              </p>
            </div>
            <div className="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2">
              <div>
                <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.defaultLanguage}
                </label>
                <SelectField
                  aria-label={t.defaultLanguage}
                  value={settings.default_language}
                  onValueChange={(val) => void patch({ default_language: val })}
                  options={languages
                    .filter((l) => l.enabled)
                    .map((l) => ({ value: l.id, label: l.name }))}
                />
              </div>

              <div>
                <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.defaultDifficulty}
                </label>
                <SelectField
                  aria-label={t.defaultDifficulty}
                  value={settings.default_difficulty}
                  onValueChange={(val) =>
                    void patch({
                      default_difficulty: val as ProblemDifficulty,
                    })
                  }
                  options={(["easy", "medium", "hard", "custom"] as const).map(
                    (d) => ({
                      value: d,
                      label: t.difficultyLabels[d],
                    }),
                  )}
                />
              </div>

              <div>
                <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.defaultStatus}
                </label>
                <SelectField
                  aria-label={t.defaultStatus}
                  value={settings.default_status}
                  onValueChange={(val) =>
                    void patch({ default_status: val as ProblemStatus })
                  }
                  options={(
                    [
                      "not_started",
                      "in_progress",
                      "solved",
                      "review",
                    ] as const
                  ).map((s) => ({
                    value: s,
                    label: t.statusLabels[s],
                  }))}
                />
              </div>

              <div>
                <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.defaultTemplateMode}
                </label>
                <SelectField
                  aria-label={t.defaultTemplateMode}
                  value={settings.default_template_mode}
                  onValueChange={(val) =>
                    void patch({
                      default_template_mode: val as TemplateMode,
                    })
                  }
                  options={[
                    { value: "starter", label: t.templateStarter },
                    { value: "empty", label: t.templateEmpty },
                  ]}
                />
              </div>
            </div>
          </Card>
        </TabsContent>

        {/* APPEARANCE */}
        <TabsContent value="appearance">
          <Card className="p-5">
            <div className="flex flex-wrap items-center justify-between gap-4">
              <div>
                <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                  {t.themeLabel}
                </h2>
                <p className="mt-0.5 text-xs text-[var(--text-secondary)]">
                  {t.themeDesc}
                </p>
              </div>
              {/* Capsule quick switcher */}
              <div className="inline-flex items-center gap-1 rounded-full border border-[var(--border-subtle)] bg-[var(--bg-subtle)] p-1">
                {(
                  [
                    { id: "system", label: t.themeSystem, icon: Monitor },
                    { id: "light", label: t.themeLight, icon: Sun },
                    { id: "dark", label: t.themeDark, icon: Moon },
                  ] as const
                ).map((opt) => {
                  const active = settings.theme === opt.id;
                  return (
                    <button
                      key={opt.id}
                      type="button"
                      onClick={() =>
                        void patch({ theme: opt.id as ThemePreference })
                      }
                      className={cn(
                        "inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-medium transition-all cursor-pointer",
                        active
                          ? "bg-[var(--bg-surface)] text-[var(--text-primary)] shadow-[var(--shadow-sm)]"
                          : "text-[var(--text-secondary)] hover:text-[var(--text-primary)]",
                      )}
                    >
                      <Icon icon={opt.icon} size={13} />
                      {opt.label}
                    </button>
                  );
                })}
              </div>
            </div>

            <div className="mt-5 grid grid-cols-1 gap-3 sm:grid-cols-3">
              {(
                [
                  {
                    id: "system",
                    label: t.themeSystem,
                    icon: Monitor,
                    preview:
                      "bg-gradient-to-r from-[#f5f5f7] from-50% to-[#1e1e22] to-50%",
                  },
                  {
                    id: "light",
                    label: t.themeLight,
                    icon: Sun,
                    preview: "bg-[#f5f5f7]",
                  },
                  {
                    id: "dark",
                    label: t.themeDark,
                    icon: Moon,
                    preview: "bg-[#1e1e22]",
                  },
                ] as const
              ).map((opt) => {
                const active = settings.theme === opt.id;
                return (
                  <button
                    key={opt.id}
                    type="button"
                    onClick={() =>
                      void patch({ theme: opt.id as ThemePreference })
                    }
                    className={cn(
                      "flex flex-col gap-3 rounded-2xl border p-3.5 text-left transition-all cursor-pointer",
                      active
                        ? "border-[var(--accent)] bg-[var(--accent-soft)]/40 ring-2 ring-[var(--ring-focus)]"
                        : "border-[var(--border-subtle)] bg-[var(--bg-surface)] hover:bg-[var(--bg-subtle)]/50",
                    )}
                  >
                    <div
                      className={cn(
                        "h-14 w-full rounded-xl border border-[var(--border-subtle)]",
                        opt.preview,
                      )}
                    />
                    <div className="flex items-center justify-between gap-2">
                      <div className="flex items-center gap-2">
                        <Icon
                          icon={opt.icon}
                          size={15}
                          className={
                            active
                              ? "text-[var(--accent-text)]"
                              : "text-[var(--text-secondary)]"
                          }
                        />
                        <span className="text-[13px] font-medium text-[var(--text-primary)]">
                          {opt.label}
                        </span>
                      </div>
                      <span
                        className={cn(
                          "h-2 w-2 rounded-full transition-colors",
                          active ? "bg-[var(--accent)]" : "bg-transparent",
                        )}
                      />
                    </div>
                  </button>
                );
              })}
            </div>
          </Card>
        </TabsContent>

        {/* WORKSPACE */}
        <TabsContent value="workspace" className="space-y-5">
          <Card className="p-5">
            <div className="flex flex-wrap items-start justify-between gap-3">
              <div>
                <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                  {t.workspacePathLabel}
                </h2>
                <p className="mt-0.5 text-xs text-[var(--text-secondary)]">
                  {t.workspacePathDesc}
                </p>
              </div>
              <Button variant="secondary" size="sm" onClick={onOpenWorkspace}>
                <Icon icon={FolderOpen} size={14} />
                {t.openWorkspace}
              </Button>
            </div>

            <form onSubmit={handleSaveWorkspace} className="mt-4 space-y-3">
              <div className="flex flex-col gap-2 sm:flex-row">
                <Input
                  value={wsInput}
                  onChange={(e) => {
                    setWsInput(e.target.value);
                    setWsValidation(null);
                  }}
                  className="font-mono text-xs"
                />
                <div className="flex shrink-0 gap-2">
                  <Button
                    variant="secondary"
                    onClick={() => void handleValidateWorkspace()}
                  >
                    {t.validatePath}
                  </Button>
                  <Button type="submit" variant="primary" disabled={wsSaving}>
                    {wsSaving ? t.saving : t.saveWorkspace}
                  </Button>
                </div>
              </div>

              {wsValidation ? (
                <div
                  className="rounded-xl border px-3.5 py-2 text-xs"
                  style={{
                    backgroundColor: wsValidation.valid
                      ? "var(--success-soft)"
                      : "var(--danger-soft)",
                    borderColor: wsValidation.valid
                      ? "var(--success)"
                      : "var(--danger)",
                    color: wsValidation.valid
                      ? "var(--success)"
                      : "var(--danger)",
                  }}
                >
                  {wsValidation.message ||
                    `${wsValidation.path} (${wsValidation.problem_folders} folders)`}
                </div>
              ) : null}
            </form>
          </Card>

          <Card className="divide-y divide-[var(--border-subtle)]">
            <div className="p-5">
              <h2 className="text-[13px] font-semibold text-[var(--text-primary)]">
                {t.workspaceStructure}
              </h2>
              <div className="mt-3 max-w-md">
                <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.fileLayout}
                </label>
                <SelectField
                  aria-label={t.fileLayout}
                  value={settings.file_layout}
                  onValueChange={(val) =>
                    void patch({ file_layout: val as FileLayout })
                  }
                  options={[
                    { value: "auto", label: t.layoutAuto },
                    { value: "subdirs", label: t.layoutSubdirs },
                    { value: "flat", label: t.layoutFlat },
                  ]}
                />
              </div>
            </div>

            <div className="flex items-center justify-between gap-4 px-5 py-4">
              <span className="text-[13px] font-medium text-[var(--text-primary)]">
                {t.groupByPlatform}
              </span>
              <Switch
                checked={settings.group_by_platform}
                onCheckedChange={(checked) =>
                  void patch({ group_by_platform: checked })
                }
                aria-label={t.groupByPlatform}
              />
            </div>

            <div className="flex items-center justify-between gap-4 px-5 py-4">
              <span className="text-[13px] font-medium text-[var(--text-primary)]">
                {t.generateReadmeOnCreate}
              </span>
              <Switch
                checked={settings.generate_readme}
                onCheckedChange={(checked) =>
                  void patch({ generate_readme: checked })
                }
                aria-label={t.generateReadmeOnCreate}
              />
            </div>
          </Card>
        </TabsContent>

        {/* LANGUAGES */}
        <TabsContent value="languages" className="space-y-4">
          <div className="flex justify-end">
            <Button variant="primary" size="sm" onClick={openCreateLanguage}>
              <Icon icon={Plus} size={14} />
              {t.addLanguage}
            </Button>
          </div>
          <Card className="divide-y divide-[var(--border-subtle)] overflow-hidden">
            {languages.map((lang) => (
              <div
                key={lang.id}
                className="flex flex-wrap items-center justify-between gap-3 px-5 py-3.5"
              >
                <div>
                  <div className="flex items-center gap-2">
                    <span className="text-[13px] font-semibold text-[var(--text-primary)]">
                      {lang.name}
                    </span>
                    <span className="rounded-full bg-[var(--bg-subtle)] px-2 py-0.5 font-mono text-[11px] text-[var(--text-muted)]">
                      .{lang.extension}
                    </span>
                    <Badge>{lang.is_builtin ? t.builtin : t.custom}</Badge>
                  </div>
                  <div className="mt-1 font-mono text-xs text-[var(--text-muted)]">
                    {lang.folder_name}/{lang.default_filename}
                  </div>
                </div>
                <div className="flex items-center gap-2.5">
                  <Switch
                    checked={lang.enabled}
                    onCheckedChange={() => void toggleLanguageEnabled(lang)}
                    aria-label={`${t.enabled}: ${lang.name}`}
                  />
                  <Button
                    size="icon"
                    variant="ghost"
                    onClick={() => openEditLanguage(lang)}
                    aria-label={t.edit}
                  >
                    <Icon icon={Pencil} size={14} />
                  </Button>
                  {!lang.is_builtin ? (
                    <Button
                      size="icon"
                      variant="ghost"
                      onClick={() => void handleDeleteLanguage(lang.id)}
                      aria-label={t.delete}
                      style={{ color: "var(--danger)" }}
                    >
                      <Icon icon={Trash2} size={14} />
                    </Button>
                  ) : null}
                </div>
              </div>
            ))}
          </Card>
        </TabsContent>

        {/* TEMPLATES */}
        <TabsContent value="templates" className="space-y-4">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <p className="max-w-2xl text-xs leading-relaxed text-[var(--text-muted)]">
              {t.readmePlaceholders}
            </p>
            <Button variant="primary" size="sm" onClick={openCreateTemplate}>
              <Icon icon={Plus} size={14} />
              {t.addTemplate}
            </Button>
          </div>

          <Card className="divide-y divide-[var(--border-subtle)] overflow-hidden">
            {templates.map((tpl) => (
              <div
                key={tpl.id}
                className="flex flex-wrap items-start justify-between gap-3 px-5 py-4"
              >
                <div className="min-w-0 flex-1">
                  <div className="flex flex-wrap items-center gap-2">
                    <span className="text-[13px] font-semibold text-[var(--text-primary)]">
                      {tpl.name}
                    </span>
                    <span className="rounded-full bg-[var(--bg-subtle)] px-2.5 py-0.5 font-mono text-[11px] text-[var(--text-secondary)]">
                      {tpl.kind === "readme"
                        ? "README.md"
                        : (tpl.language_id ?? "source")}
                    </span>
                    {tpl.is_default ? (
                      <span
                        className="rounded-full px-2.5 py-0.5 text-[10px] font-medium"
                        style={{
                          backgroundColor: "var(--accent-soft)",
                          color: "var(--accent-text)",
                        }}
                      >
                        {t.defaultBadge}
                      </span>
                    ) : null}
                  </div>
                  <pre className="mt-2.5 max-h-24 overflow-hidden rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-canvas)] px-3.5 py-2.5 font-mono text-[11px] leading-relaxed text-[var(--text-muted)]">
                    {tpl.content}
                  </pre>
                </div>

                <div className="flex shrink-0 items-center gap-1.5">
                  <Button
                    size="sm"
                    variant="secondary"
                    onClick={() => openEditTemplate(tpl)}
                  >
                    <Icon icon={Pencil} size={13} />
                    {t.edit}
                  </Button>
                  {!tpl.is_builtin ? (
                    <Button
                      size="icon"
                      variant="ghost"
                      onClick={() => void handleDeleteTemplate(tpl.id)}
                      aria-label={t.delete}
                      style={{ color: "var(--danger)" }}
                    >
                      <Icon icon={Trash2} size={14} />
                    </Button>
                  ) : null}
                </div>
              </div>
            ))}
          </Card>
        </TabsContent>
      </Tabs>

      {/* Language Modal */}
      <Dialog
        open={langModalOpen}
        onClose={() => setLangModalOpen(false)}
        title={editingLang ? t.edit : t.addLanguage}
        maxWidth="max-w-md"
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={() => setLangModalOpen(false)}>
              {t.cancel}
            </Button>
            <Button type="submit" form="ducky-lang-form" variant="primary">
              {t.save}
            </Button>
          </div>
        }
      >
        <form
          id="ducky-lang-form"
          onSubmit={submitLanguage}
          className="space-y-3.5"
        >
          {langError ? (
            <div
              className="rounded-xl border px-3 py-2 text-xs"
              style={{
                backgroundColor: "var(--danger-soft)",
                borderColor: "var(--danger)",
                color: "var(--danger)",
              }}
            >
              {langError}
            </div>
          ) : null}

          {!editingLang ? (
            <div>
              <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                {t.languageId}
              </label>
              <Input
                required
                value={langId}
                onChange={(e) => setLangId(e.target.value)}
                placeholder="e.g. go, ts"
                className="font-mono text-xs"
              />
            </div>
          ) : null}

          <div>
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.languageName}
            </label>
            <Input
              required
              value={langName}
              onChange={(e) => setLangName(e.target.value)}
              placeholder="Go"
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                {t.fileExtension}
              </label>
              <Input
                required
                value={langExt}
                onChange={(e) => setLangExt(e.target.value)}
                placeholder="go"
                className="font-mono text-xs"
              />
            </div>
            <div>
              <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                {t.folderName}
              </label>
              <Input
                required
                value={langFolder}
                onChange={(e) => setLangFolder(e.target.value)}
                placeholder="go"
                className="font-mono text-xs"
              />
            </div>
          </div>

          <div>
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.defaultFilename}
            </label>
            <Input
              required
              value={langFile}
              onChange={(e) => setLangFile(e.target.value)}
              placeholder="main.go"
              className="font-mono text-xs"
            />
          </div>
        </form>
      </Dialog>

      {/* Template Modal */}
      <Dialog
        open={tplModalOpen}
        onClose={() => setTplModalOpen(false)}
        title={editingTpl ? t.edit : t.addTemplate}
        maxWidth="max-w-xl"
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={() => setTplModalOpen(false)}>
              {t.cancel}
            </Button>
            <Button type="submit" form="ducky-tpl-form" variant="primary">
              {t.save}
            </Button>
          </div>
        }
      >
        <form
          id="ducky-tpl-form"
          onSubmit={submitTemplate}
          className="space-y-3.5"
        >
          {tplError ? (
            <div
              className="rounded-xl border px-3 py-2 text-xs"
              style={{
                backgroundColor: "var(--danger-soft)",
                borderColor: "var(--danger)",
                color: "var(--danger)",
              }}
            >
              {tplError}
            </div>
          ) : null}

          <div>
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.templateName}
            </label>
            <Input
              required
              value={tplName}
              onChange={(e) => setTplName(e.target.value)}
            />
          </div>

          {!editingTpl ? (
            <div className="grid grid-cols-2 gap-3">
              <div>
                <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                  {t.templateKind}
                </label>
                <SelectField
                  value={tplKind}
                  onValueChange={(v) => setTplKind(v as "source" | "readme")}
                  options={[
                    { value: "source", label: t.templateKindSource },
                    { value: "readme", label: t.templateKindReadme },
                  ]}
                />
              </div>
              {tplKind === "source" ? (
                <div>
                  <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
                    {t.language}
                  </label>
                  <SelectField
                    value={tplLangId}
                    onValueChange={setTplLangId}
                    options={languages.map((l) => ({
                      value: l.id,
                      label: l.name,
                    }))}
                  />
                </div>
              ) : null}
            </div>
          ) : null}

          <div>
            <label className="mb-1.5 block text-xs font-medium text-[var(--text-secondary)]">
              {t.templateContent}
            </label>
            <Textarea
              rows={10}
              value={tplContent}
              onChange={(e) => setTplContent(e.target.value)}
              className="font-mono text-xs"
            />
          </div>

          <label className="flex items-center gap-2 text-xs text-[var(--text-secondary)] cursor-pointer">
            <Checkbox checked={tplDefault} onCheckedChange={setTplDefault} />
            <span>{t.setAsDefault}</span>
          </label>
        </form>
      </Dialog>
    </div>
  );
}

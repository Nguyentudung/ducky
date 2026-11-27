import type {
  AppSettings,
  CreateLanguageInput,
  CreateProblemInput,
  CreateTemplateInput,
  DashboardResponse,
  HealthResponse,
  Language,
  Problem,
  ProblemDetail,
  ProblemQuery,
  Template,
  UpdateLanguageInput,
  UpdateProblemInput,
  UpdateTemplateInput,
  WorkspaceFile,
  WorkspaceStatus,
} from "../types/api";

export class ApiError extends Error {
  readonly code: string;
  readonly status: number;

  constructor(message: string, code = "api_error", status = 500) {
    super(message);
    this.code = code;
    this.status = status;
  }
}

async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const headers = new Headers(init.headers);
  headers.set("X-Ducky-Request", "1");
  if (init.body && !headers.has("Content-Type")) {
    headers.set("Content-Type", "application/json");
  }

  const res = await fetch(path, {
    ...init,
    headers,
    credentials: "same-origin",
  });

  const text = await res.text();
  let parsed: unknown = null;
  if (text) {
    try {
      parsed = JSON.parse(text);
    } catch {
      parsed = null;
    }
  }

  if (!res.ok) {
    const errObj =
      parsed && typeof parsed === "object" && "error" in parsed
        ? (parsed as { error?: { code?: string; message?: string } }).error
        : undefined;
    throw new ApiError(
      errObj?.message || `Request failed (${res.status})`,
      errObj?.code || "http_error",
      res.status,
    );
  }

  return parsed as T;
}

export const api = {
  health: () => request<HealthResponse>("/api/health"),

  dashboard: () => request<DashboardResponse>("/api/dashboard"),

  listProblems: (query: ProblemQuery = {}) => {
    const params = new URLSearchParams();
    for (const [k, v] of Object.entries(query)) {
      if (v !== undefined && v !== "") {
        params.set(k, String(v));
      }
    }
    const qs = params.toString();
    return request<{ problems: Problem[] }>(
      qs ? `/api/problems?${qs}` : "/api/problems",
    ).then((r) => r.problems);
  },

  createProblem: (input: CreateProblemInput) =>
    request<ProblemDetail>("/api/problems", {
      method: "POST",
      body: JSON.stringify(input),
    }),

  getProblem: (id: number, recordOpen = false) =>
    request<ProblemDetail>(
      recordOpen ? `/api/problems/${id}?open=1` : `/api/problems/${id}`,
    ),

  updateProblem: (id: number, input: UpdateProblemInput) =>
    request<{ problem: Problem }>(`/api/problems/${id}`, {
      method: "PATCH",
      body: JSON.stringify(input),
    }).then((r) => r.problem),

  deleteProblem: (id: number, deleteFiles = false) =>
    request<{ deleted: boolean; files_deleted: boolean }>(
      `/api/problems/${id}?delete_files=${deleteFiles ? "true" : "false"}`,
      { method: "DELETE" },
    ),

  addProblemFile: (
    id: number,
    input: {
      language_id: string;
      filename?: string;
      template_id?: string;
      subdir?: boolean;
    },
  ) =>
    request<{ file: WorkspaceFile }>(`/api/problems/${id}/files`, {
      method: "POST",
      body: JSON.stringify(input),
    }).then((r) => r.file),

  generateReadme: (
    id: number,
    input: { overwrite: boolean; template_id?: string },
  ) =>
    request<{ generated: boolean }>(`/api/problems/${id}/readme`, {
      method: "POST",
      body: JSON.stringify(input),
    }),

  openProblemFolder: (id: number) =>
    request<{ opened: boolean }>(`/api/problems/${id}/open-folder`, {
      method: "POST",
    }),

  clearRecent: () =>
    request<{ cleared: boolean }>("/api/recent", { method: "DELETE" }),

  listLanguages: () =>
    request<{ languages: Language[] }>("/api/languages").then((r) => r.languages),

  createLanguage: (input: CreateLanguageInput) =>
    request<{ language: Language }>("/api/languages", {
      method: "POST",
      body: JSON.stringify(input),
    }).then((r) => r.language),

  updateLanguage: (id: string, input: UpdateLanguageInput) =>
    request<{ language: Language }>(`/api/languages/${encodeURIComponent(id)}`, {
      method: "PATCH",
      body: JSON.stringify(input),
    }).then((r) => r.language),

  deleteLanguage: (id: string) =>
    request<{ deleted: boolean }>(`/api/languages/${encodeURIComponent(id)}`, {
      method: "DELETE",
    }),

  listTemplates: () =>
    request<{ templates: Template[] }>("/api/templates").then((r) => r.templates),

  createTemplate: (input: CreateTemplateInput) =>
    request<{ template: Template }>("/api/templates", {
      method: "POST",
      body: JSON.stringify(input),
    }).then((r) => r.template),

  updateTemplate: (id: string, input: UpdateTemplateInput) =>
    request<{ template: Template }>(`/api/templates/${encodeURIComponent(id)}`, {
      method: "PATCH",
      body: JSON.stringify(input),
    }).then((r) => r.template),

  deleteTemplate: (id: string) =>
    request<{ deleted: boolean }>(`/api/templates/${encodeURIComponent(id)}`, {
      method: "DELETE",
    }),

  getSettings: () =>
    request<{ settings: AppSettings }>("/api/settings").then((r) => r.settings),

  patchSettings: (patch: Partial<AppSettings>) =>
    request<{ settings: AppSettings }>("/api/settings", {
      method: "PATCH",
      body: JSON.stringify(patch),
    }).then((r) => r.settings),

  getWorkspace: () =>
    request<{ workspace: WorkspaceStatus }>("/api/workspace").then(
      (r) => r.workspace,
    ),

  validateWorkspace: (path: string) =>
    request<{ workspace: WorkspaceStatus }>("/api/workspace/validate", {
      method: "POST",
      body: JSON.stringify({ path }),
    }).then((r) => r.workspace),

  openWorkspace: () =>
    request<{ opened: boolean }>("/api/workspace/open", { method: "POST" }),
};

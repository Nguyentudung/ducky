export type ProblemStatus = "not_started" | "in_progress" | "solved" | "review";
export type ProblemDifficulty = "easy" | "medium" | "hard" | "custom";
export type FileLayout = "auto" | "subdirs" | "flat";
export type TemplateMode = "starter" | "empty";
export type ThemePreference = "system" | "light" | "dark";
export type LanguagePreference = "system" | "en" | "vi";

export interface Problem {
  id: number;
  identifier: string;
  title: string;
  description: string;
  platform: string;
  url: string;
  difficulty: ProblemDifficulty;
  tags: string[];
  status: ProblemStatus;
  languages: string[];
  workspace_path: string;
  folder_name: string;
  notes: string;
  created_at: string;
  updated_at: string;
  last_opened_at: string | null;
}

export interface WorkspaceFile {
  name: string;
  relative_path: string;
  language: string | null;
  size_bytes: number;
  is_readme: boolean;
}

export interface ProblemDetail {
  problem: Problem;
  files: WorkspaceFile[];
  workspace_exists: boolean;
}

export interface CreateProblemInput {
  identifier: string;
  title: string;
  description: string;
  platform: string;
  url: string;
  difficulty: ProblemDifficulty;
  tags: string[];
  status: ProblemStatus;
  languages: string[];
  notes: string;
  folder_name?: string;
  workspace_root?: string;
  group_by_platform?: boolean;
  file_layout?: FileLayout;
  template_mode?: string;
  custom_filenames?: Record<string, string>;
  generate_readme?: boolean;
}

export interface UpdateProblemInput {
  identifier?: string;
  title?: string;
  description?: string;
  platform?: string;
  url?: string;
  difficulty?: ProblemDifficulty;
  tags?: string[];
  status?: ProblemStatus;
  languages?: string[];
  notes?: string;
}

export interface ProblemQuery {
  search?: string;
  status?: string;
  difficulty?: string;
  language?: string;
  platform?: string;
  tag?: string;
  sort?: "modified" | "created" | "title" | "difficulty" | "status";
  dir?: "asc" | "desc";
}

export interface DashboardStats {
  total: number;
  solved: number;
  in_progress: number;
  not_started: number;
  review: number;
  completion_percent: number;
  recent: Problem[];
  modified: Problem[];
  platforms: string[];
  tags: string[];
}

export interface WorkspaceStatus {
  path: string;
  exists: boolean;
  writable: boolean;
  valid: boolean;
  problem_folders: number;
  message: string | null;
}

export interface DashboardResponse {
  stats: DashboardStats;
  workspace: WorkspaceStatus;
}

export interface Language {
  id: string;
  name: string;
  extension: string;
  default_filename: string;
  folder_name: string;
  is_builtin: boolean;
  enabled: boolean;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface CreateLanguageInput {
  id: string;
  name: string;
  extension: string;
  default_filename: string;
  folder_name: string;
}

export interface UpdateLanguageInput {
  name?: string;
  extension?: string;
  default_filename?: string;
  folder_name?: string;
  enabled?: boolean;
  sort_order?: number;
}

export interface Template {
  id: string;
  name: string;
  kind: "source" | "readme";
  language_id: string | null;
  content: string;
  is_default: boolean;
  is_builtin: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateTemplateInput {
  name: string;
  kind: "source" | "readme";
  language_id?: string | null;
  content: string;
  is_default?: boolean;
}

export interface UpdateTemplateInput {
  name?: string;
  content?: string;
  is_default?: boolean;
}

export interface AppSettings {
  workspace_path: string;
  theme: ThemePreference;
  language: LanguagePreference;
  default_language: string;
  default_difficulty: ProblemDifficulty;
  default_status: ProblemStatus;
  default_template_mode: TemplateMode;
  group_by_platform: boolean;
  file_layout: FileLayout;
  generate_readme: boolean;
}

export interface HealthResponse {
  status: string;
  version: string;
  host: string;
  port: number;
  database: boolean;
}

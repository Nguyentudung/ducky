import { useCallback, useEffect, useMemo, useState } from "react";
import { api } from "./services/api";
import { resolveLocale, translations } from "./i18n/translations";
import { useTheme } from "./hooks/useTheme";
import { ToastProvider, useToast } from "./hooks/useToast";
import { Layout, type Route } from "./components/Layout";
import { ProblemFormModal } from "./components/ProblemFormModal";
import { ErrorBanner, LoadingState } from "./components/ui";
import { HomePage } from "./pages/HomePage";
import { ProblemsPage } from "./pages/ProblemsPage";
import { ProblemDetailPage } from "./pages/ProblemDetailPage";
import { SettingsPage } from "./pages/SettingsPage";
import type {
  AppSettings,
  CreateProblemInput,
  Language,
  Template,
  WorkspaceStatus,
} from "./types/api";

function parseRoute(pathname: string): Route {
  const clean = pathname.replace(/\/+$/, "") || "/";
  if (clean === "/problems") return { page: "problems" };
  const problemMatch = clean.match(/^\/problems\/(\d+)$/);
  if (problemMatch) {
    return { page: "problem", id: Number(problemMatch[1]) };
  }
  if (clean === "/settings") return { page: "settings" };
  const settingsMatch = clean.match(/^\/settings\/([a-z]+)$/);
  if (settingsMatch) {
    return { page: "settings", tab: settingsMatch[1] };
  }
  return { page: "home" };
}

function routeToPath(r: Route): string {
  switch (r.page) {
    case "home":
      return "/";
    case "problems":
      return "/problems";
    case "problem":
      return `/problems/${r.id}`;
    case "settings":
      return r.tab ? `/settings/${r.tab}` : "/settings";
  }
}

function DuckyAppShell() {
  const { pushToast } = useToast();
  const [route, setRoute] = useState<Route>(() =>
    parseRoute(window.location.pathname),
  );
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [languages, setLanguages] = useState<Language[]>([]);
  const [templates, setTemplates] = useState<Template[]>([]);
  const [workspace, setWorkspace] = useState<WorkspaceStatus | null>(null);
  const [bootError, setBootError] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [refreshKey, setRefreshKey] = useState(0);

  const locale = useMemo(
    () => resolveLocale(settings?.language ?? "system"),
    [settings?.language],
  );
  const t = translations[locale];

  useTheme(settings?.theme ?? "system");

  const bootstrap = useCallback(async () => {
    setBootError(null);
    try {
      const [s, l, tpl, ws] = await Promise.all([
        api.getSettings(),
        api.listLanguages(),
        api.listTemplates(),
        api.getWorkspace(),
      ]);
      setSettings(s);
      setLanguages(l);
      setTemplates(tpl);
      setWorkspace(ws);
    } catch (err) {
      setBootError(
        err instanceof Error ? err.message : "Could not connect to Ducky server",
      );
    }
  }, []);

  useEffect(() => {
    void bootstrap();
  }, [bootstrap]);

  useEffect(() => {
    const onPop = () => setRoute(parseRoute(window.location.pathname));
    window.addEventListener("popstate", onPop);
    return () => window.removeEventListener("popstate", onPop);
  }, []);

  const navigate = useCallback((next: Route) => {
    const path = routeToPath(next);
    if (window.location.pathname !== path) {
      window.history.pushState(null, "", path);
    }
    setRoute(next);
  }, []);

  const refreshWorkspace = useCallback(async () => {
    try {
      const ws = await api.getWorkspace();
      setWorkspace(ws);
    } catch {
      // ignore transient error
    }
  }, []);

  const handleOpenWorkspace = useCallback(async () => {
    try {
      await api.openWorkspace();
      pushToast(t.toastFolderOpened, "info");
    } catch (err) {
      pushToast(
        err instanceof Error ? err.message : "Failed to open workspace",
        "error",
      );
    }
  }, [pushToast, t.toastFolderOpened]);

  const handleCreateProblem = async (input: CreateProblemInput) => {
    const created = await api.createProblem(input);
    pushToast(t.toastProblemCreated, "success");
    setRefreshKey((k) => k + 1);
    void refreshWorkspace();
    navigate({ page: "problem", id: created.problem.id });
  };

  if (bootError) {
    return (
      <div className="mx-auto max-w-lg px-4 py-20">
        <ErrorBanner message={bootError} onRetry={() => void bootstrap()} />
      </div>
    );
  }

  if (!settings) {
    return <LoadingState label="Starting Ducky…" />;
  }

  return (
    <Layout
      route={route}
      onNavigate={navigate}
      onNewProblem={() => setCreateOpen(true)}
      onOpenWorkspace={() => void handleOpenWorkspace()}
      workspace={workspace}
      t={t}
    >
      {route.page === "home" ? (
        <HomePage
          t={t}
          refreshKey={refreshKey}
          onNavigate={navigate}
          onNewProblem={() => setCreateOpen(true)}
          onOpenWorkspace={() => void handleOpenWorkspace()}
        />
      ) : null}

      {route.page === "problems" ? (
        <ProblemsPage
          t={t}
          languages={languages}
          refreshKey={refreshKey}
          onNavigate={navigate}
          onNewProblem={() => setCreateOpen(true)}
          onChanged={() => setRefreshKey((k) => k + 1)}
        />
      ) : null}

      {route.page === "problem" ? (
        <ProblemDetailPage
          problemId={route.id}
          settings={settings}
          languages={languages}
          templates={templates}
          t={t}
          onNavigate={navigate}
          onChanged={() => {
            setRefreshKey((k) => k + 1);
            void refreshWorkspace();
          }}
        />
      ) : null}

      {route.page === "settings" ? (
        <SettingsPage
          initialTab={route.tab}
          settings={settings}
          languages={languages}
          templates={templates}
          t={t}
          onSettingsUpdated={(next) => {
            setSettings(next);
            void refreshWorkspace();
          }}
          onLanguagesChanged={async () => {
            setLanguages(await api.listLanguages());
          }}
          onTemplatesChanged={async () => {
            setTemplates(await api.listTemplates());
          }}
          onOpenWorkspace={() => void handleOpenWorkspace()}
        />
      ) : null}

      <ProblemFormModal
        open={createOpen}
        onClose={() => setCreateOpen(false)}
        mode="create"
        settings={settings}
        languages={languages}
        templates={templates}
        t={t}
        onCreate={handleCreateProblem}
        onUpdate={async () => {}}
      />
    </Layout>
  );
}

export default function App() {
  return (
    <ToastProvider>
      <DuckyAppShell />
    </ToastProvider>
  );
}

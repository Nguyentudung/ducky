import React, { useState } from "react";
import {
  Code2,
  FolderOpen,
  Home,
  Menu,
  Plus,
  Settings,
  X,
  type LucideIcon,
} from "lucide-react";
import { Icon } from "./Icon";
import { Button, Tooltip, cn } from "./ui";
import type { TranslationDict } from "../i18n/translations";
import type { WorkspaceStatus } from "../types/api";

export type Route =
  | { page: "home" }
  | { page: "problems" }
  | { page: "problem"; id: number }
  | { page: "settings"; tab?: string };

interface LayoutProps {
  route: Route;
  onNavigate: (r: Route) => void;
  onNewProblem: () => void;
  onOpenWorkspace: () => void;
  workspace: WorkspaceStatus | null;
  t: TranslationDict;
  children: React.ReactNode;
}

export function Layout({
  route,
  onNavigate,
  onNewProblem,
  onOpenWorkspace,
  workspace,
  t,
  children,
}: LayoutProps) {
  const [mobileOpen, setMobileOpen] = useState(false);

  const navItems: Array<{
    key: "home" | "problems" | "settings";
    label: string;
    icon: LucideIcon;
    active: boolean;
    target: Route;
  }> = [
    {
      key: "home",
      label: t.navHome,
      icon: Home,
      active: route.page === "home",
      target: { page: "home" },
    },
    {
      key: "problems",
      label: t.navProblems,
      icon: Code2,
      active: route.page === "problems" || route.page === "problem",
      target: { page: "problems" },
    },
    {
      key: "settings",
      label: t.navSettings,
      icon: Settings,
      active: route.page === "settings",
      target: { page: "settings" },
    },
  ];

  const sidebarContent = (
    <div className="flex h-full flex-col justify-between px-3 py-4">
      <div>
        {/* Brand Identity */}
        <div className="flex items-center justify-between px-2 pb-5">
          <button
            type="button"
            onClick={() => {
              onNavigate({ page: "home" });
              setMobileOpen(false);
            }}
            className="flex items-center gap-2.5 text-left cursor-pointer group"
          >
            <img
              src="/logo.png"
              alt="Ducky"
              className="h-8 w-8 rounded-lg object-contain transition-transform duration-150 group-hover:scale-[1.03]"
            />
            <div className="min-w-0">
              <div className="text-[14px] font-semibold tracking-tight text-[var(--text-primary)] leading-tight">
                {t.appName}
              </div>
              <div className="text-[11px] text-[var(--text-muted)] leading-snug mt-0.5">
                {t.tagline}
              </div>
            </div>
          </button>
          <button
            type="button"
            className="rounded-full p-1.5 text-[var(--text-muted)] hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)] lg:hidden cursor-pointer"
            onClick={() => setMobileOpen(false)}
            aria-label={t.close}
          >
            <Icon icon={X} size={17} />
          </button>
        </div>

        {/* Primary Action */}
        <div className="px-1 pb-4">
          <Button
            variant="primary"
            className="w-full justify-center"
            onClick={() => {
              setMobileOpen(false);
              onNewProblem();
            }}
          >
            <Icon icon={Plus} size={15} strokeWidth={2} />
            {t.newProblem}
          </Button>
        </div>

        {/* Navigation */}
        <nav className="space-y-1 px-1" aria-label="Main navigation">
          {navItems.map((item) => (
            <button
              key={item.key}
              type="button"
              onClick={() => {
                onNavigate(item.target);
                setMobileOpen(false);
              }}
              className={cn(
                "flex w-full items-center gap-2.5 rounded-full px-3.5 py-2 text-[13px] font-medium transition-colors cursor-pointer",
                item.active
                  ? "bg-[var(--accent-soft)] text-[var(--accent-text)]"
                  : "text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)]",
              )}
            >
              <Icon
                icon={item.icon}
                size={16}
                strokeWidth={item.active ? 2 : 1.75}
              />
              <span>{item.label}</span>
            </button>
          ))}
        </nav>
      </div>

      {/* Compact Workspace Footer */}
      <div className="border-t border-[var(--border-subtle)] px-2 pt-3">
        <div className="flex items-center justify-between gap-2">
          <button
            type="button"
            onClick={() => {
              onNavigate({ page: "settings", tab: "workspace" });
              setMobileOpen(false);
            }}
            className="flex min-w-0 flex-1 items-center gap-2 text-left cursor-pointer group"
          >
            <span
              className="h-2 w-2 shrink-0 rounded-full"
              style={{
                backgroundColor: workspace?.valid
                  ? "var(--success)"
                  : "var(--warning)",
              }}
              aria-hidden="true"
            />
            <div className="min-w-0 flex-1">
              <div className="flex items-center gap-1.5">
                <span className="text-[11px] font-medium text-[var(--text-secondary)] group-hover:text-[var(--text-primary)]">
                  {t.workspace}
                </span>
                <span
                  className="text-[10px] font-medium"
                  style={{
                    color: workspace?.valid
                      ? "var(--success)"
                      : "var(--warning)",
                  }}
                >
                  {workspace?.valid ? t.workspaceHealthy : t.workspaceWarning}
                </span>
              </div>
              <p
                className="truncate font-mono text-[11px] text-[var(--text-muted)]"
                title={workspace?.path || ""}
              >
                {workspace?.path || "—"}
              </p>
            </div>
          </button>

          <Tooltip content={t.openWorkspace}>
            <button
              type="button"
              onClick={onOpenWorkspace}
              aria-label={t.openWorkspace}
              className="shrink-0 rounded-full p-1.5 text-[var(--text-muted)] transition-colors hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)] cursor-pointer"
            >
              <Icon icon={FolderOpen} size={15} />
            </button>
          </Tooltip>
        </div>
      </div>
    </div>
  );

  return (
    <div className="flex min-h-screen bg-[var(--bg-canvas)] text-[var(--text-primary)]">
      {/* Desktop Sidebar */}
      <aside className="hidden w-[244px] shrink-0 border-r border-[var(--border-subtle)] bg-[var(--bg-sidebar)] lg:block">
        <div className="sticky top-0 h-screen">{sidebarContent}</div>
      </aside>

      {/* Mobile Drawer */}
      {mobileOpen ? (
        <div
          className="fixed inset-0 z-40 flex bg-black/35 backdrop-blur-[2px] lg:hidden"
          onClick={() => setMobileOpen(false)}
        >
          <aside
            className="h-full w-64 border-r border-[var(--border-subtle)] bg-[var(--bg-sidebar)]"
            onClick={(e) => e.stopPropagation()}
          >
            {sidebarContent}
          </aside>
        </div>
      ) : null}

      {/* Main Content Area */}
      <div className="flex min-w-0 flex-1 flex-col">
        <header className="sticky top-0 z-30 flex items-center justify-between border-b border-[var(--border-subtle)] bg-[var(--bg-surface)]/90 px-4 py-2.5 backdrop-blur-md lg:hidden">
          <div className="flex items-center gap-2.5">
            <button
              type="button"
              onClick={() => setMobileOpen(true)}
              aria-label="Open menu"
              className="rounded-lg p-1.5 text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)] cursor-pointer"
            >
              <Icon icon={Menu} size={19} />
            </button>
            <img
              src="/logo.png"
              alt="Ducky"
              className="h-6 w-6 rounded-md object-contain"
            />
            <span className="text-[14px] font-semibold tracking-tight">
              {t.appName}
            </span>
          </div>
          <Button size="sm" variant="primary" onClick={onNewProblem}>
            <Icon icon={Plus} size={14} />
            {t.newProblem}
          </Button>
        </header>

        <main className="mx-auto w-full max-w-5xl flex-1 px-5 py-7 sm:px-8 lg:px-10 lg:py-9">
          {children}
        </main>
      </div>
    </div>
  );
}

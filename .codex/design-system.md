# Ducky Design System

## 1. Visual Direction

Calm, crisp, purposeful workspace software for programming learners:
- Neutral paper/slate surfaces with one restrained cobalt-blue primary action accent and subtle warm-amber duck identity mark (used sparingly for the brand emblem, never loud).
- Generous breathing room, crisp 1px hairline borders, minimal elevation (no heavy drop shadows, no glassmorphism, no gratuitous gradients).
- Typography: clean sans-serif UI (`Inter` / system UI stack) paired with monospace (`JetBrains Mono` / system monospace stack) for identifiers, paths, filenames, and template code.

## 2. Color Tokens

### Light Theme
- `--bg-canvas`: `#F7F8FA` (page backdrop)
- `--bg-surface`: `#FFFFFF` (cards, panels, sidebar)
- `--bg-subtle`: `#F0F2F5` (inputs, hover fills, secondary surfaces)
- `--bg-muted`: `#E6E9EF` (pressed / active pills)
- `--border-subtle`: `#E4E7EC`
- `--border-strong`: `#CFD5DF`
- `--text-primary`: `#111827`
- `--text-secondary`: `#4B5563`
- `--text-muted`: `#6B7280`
- `--accent`: `#2563EB` (primary actions)
- `--accent-hover`: `#1D4ED8`
- `--accent-soft`: `#EFF6FF`
- `--accent-text`: `#1D4ED8`
- `--success`: `#15803D` / soft `#ECFDF3`
- `--warning`: `#B45309` / soft `#FFFBEB`
- `--danger`: `#B91C1C` / soft `#FEF2F2`
- `--info`: `#0369A1` / soft `#F0F9FF`

### Dark Theme (True Dark Charcoal, Not Muddy Blue-Gray)
- `--bg-canvas`: `#0D0F12`
- `--bg-surface`: `#14171C`
- `--bg-subtle`: `#1C2027`
- `--bg-muted`: `#252A33`
- `--border-subtle`: `#242933`
- `--border-strong`: `#323946`
- `--text-primary`: `#F3F4F6`
- `--text-secondary`: `#A6B0C0`
- `--text-muted`: `#7B8596`
- `--accent`: `#3B82F6`
- `--accent-hover`: `#60A5FA`
- `--accent-soft`: `rgba(59, 130, 246, 0.14)`
- `--accent-text`: `#93C5FD`
- `--success`: `#4ADE80` / soft `rgba(34, 197, 94, 0.14)`
- `--warning`: `#FBBF24` / soft `rgba(245, 158, 11, 0.14)`
- `--danger`: `#F87171` / soft `rgba(239, 68, 68, 0.14)`
- `--info`: `#38BDF8` / soft `rgba(56, 189, 248, 0.14)`

## 3. Layout & Navigation

- **Left Sidebar** (`236px` desktop, collapsible drawer on mobile/tablet):
  - Ducky brand header + quick "New Problem" CTA.
  - Primary nav: **Home**, **Problems**, **Settings**.
  - Compact workspace indicator at the bottom showing configured workspace name + quick open folder button.
- **Main Content Area**:
  - Max width `1120px` centered with `24px–32px` padding.
  - Responsive layout tested at desktop (`1440x900`) and mobile (`390x844`).
- **Icons**:
  - `@hugeicons/react` + `@hugeicons/core-free-icons` used exclusively across the entire UI.

## 4. Core Views & Interactions

1. **Home**: Compact practice summary (Total, Solved, In Progress, Review, Completion bar), Quick Actions ("New Problem", "Open Workspace", "Browse Problems"), Recently Opened problems, and Recently Modified problems.
2. **Problems**: Full search (`title`, `identifier`, `platform`, `tags`), filters (`status`, `difficulty`, `language`, `platform`), sort (`updated`, `created`, `title`, `difficulty`, `status`), quick status toggle, view detail, open folder, and empty/no-results states.
3. **Create Problem Modal**: Clean essential fields first (Title, Platform, Languages, Status, Difficulty) with an expandable "Advanced & File Generation" disclosure section (Identifier, URL, Custom Folder Name, Group by Platform, File Layout, Template, Description, Notes).
4. **Problem Detail Page**: Metadata header with inline quick status selector, Open Folder & Open Source URL actions, Edit dialog, Delete confirmation dialog (explicitly clarifying files remain safe on disk unless user opts in), Notes & Description view/edit, Generated Files list with "Add Language File" & "Generate/Update README" dialogs.
5. **Settings Page**: Tabbed sections so nothing feels overwhelming:
   - **General**: App language (`System`, `English`, `Vietnamese`), Default problem preferences (`language`, `difficulty`, `status`, `template mode`).
   - **Appearance**: Theme (`System`, `Light`, `Dark`) with live instant preview.
   - **Workspace**: Workspace root path with live validation (`Validate`), layout options (`group_by_platform`, `file_layout`), Open Workspace button.
   - **Languages**: Built-in and custom programming languages (name, extension, folder name, default filename, enabled toggle, add/edit/delete custom).
   - **Templates**: Source file templates per language and README template editor with placeholder reference (`{{title}}`, `{{platform}}`, etc.).

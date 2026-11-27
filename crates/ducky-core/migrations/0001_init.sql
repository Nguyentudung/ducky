-- Ducky SQLite Schema (v1)
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS languages (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    default_filename TEXT NOT NULL,
    folder_name TEXT NOT NULL,
    is_builtin INTEGER NOT NULL DEFAULT 0 CHECK (is_builtin IN (0, 1)),
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('source', 'readme')),
    language_id TEXT REFERENCES languages(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0 CHECK (is_default IN (0, 1)),
    is_builtin INTEGER NOT NULL DEFAULT 0 CHECK (is_builtin IN (0, 1)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_templates_kind_lang
    ON templates(kind, language_id);

CREATE TABLE IF NOT EXISTS problems (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier TEXT NOT NULL DEFAULT '',
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    platform TEXT NOT NULL DEFAULT '',
    url TEXT NOT NULL DEFAULT '',
    difficulty TEXT NOT NULL DEFAULT 'medium',
    tags TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'not_started',
    languages TEXT NOT NULL DEFAULT '[]',
    workspace_path TEXT NOT NULL,
    folder_name TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    last_opened_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_problems_identifier_unique
    ON problems(identifier) WHERE identifier != '';
CREATE INDEX IF NOT EXISTS idx_problems_updated_at
    ON problems(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_problems_status
    ON problems(status);

CREATE TABLE IF NOT EXISTS recent_problems (
    problem_id INTEGER PRIMARY KEY REFERENCES problems(id) ON DELETE CASCADE,
    opened_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_recent_problems_opened_at
    ON recent_problems(opened_at DESC);

-- Seed built-in languages
INSERT OR IGNORE INTO languages (id, name, extension, default_filename, folder_name, is_builtin, enabled, sort_order) VALUES
    ('cpp',    'C++',    'cpp',  'main.cpp',    'cpp',    1, 1, 1),
    ('c',      'C',      'c',    'main.c',      'c',      1, 1, 2),
    ('python', 'Python', 'py',   'solution.py', 'python', 1, 1, 3),
    ('rust',   'Rust',   'rs',   'main.rs',     'rust',   1, 1, 4),
    ('java',   'Java',   'java', 'Main.java',   'java',   1, 1, 5);

-- Seed built-in source and README templates
INSERT OR IGNORE INTO templates (id, name, kind, language_id, content, is_default, is_builtin) VALUES
    (
        'starter-c',
        'C Starter',
        'source',
        'c',
        '#include <stdio.h>

int main(void) {
    return 0;
}
',
        1,
        1
    ),
    (
        'starter-cpp',
        'C++ Competitive Starter',
        'source',
        'cpp',
        '#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    return 0;
}
',
        1,
        1
    ),
    (
        'starter-python',
        'Python Starter',
        'source',
        'python',
        'def solve() -> None:
    pass


if __name__ == "__main__":
    solve()
',
        1,
        1
    ),
    (
        'starter-rust',
        'Rust Starter',
        'source',
        'rust',
        'fn main() {
    // Write your solution here.
}
',
        1,
        1
    ),
    (
        'starter-java',
        'Java Starter',
        'source',
        'java',
        'import java.io.*;
import java.util.*;

public class Main {
    public static void main(String[] args) throws Exception {
        // Write your solution here.
    }
}
',
        1,
        1
    ),
    (
        'readme-default',
        'Default Problem README',
        'readme',
        NULL,
        '# {{title}}

- **Platform:** {{platform}}
- **Difficulty:** {{difficulty}}
- **Status:** {{status}}
- **URL:** {{url}}
- **Languages:** {{languages}}
- **Tags:** {{tags}}

## Description

{{description}}

## Notes

{{notes}}
',
        1,
        1
    );

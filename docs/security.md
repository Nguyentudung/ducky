# Security & Workspace Safety

## Local HTTP Server Protections

- **Loopback binding**: Binds exclusively to `127.0.0.1` by default (never `0.0.0.0`).
- **Host header validation**: Rejects requests with unrecognized `Host` headers to prevent DNS rebinding attacks.
- **CSRF / Cross-origin guard**:
  - No wildcard CORS headers are emitted.
  - Rejects `/api/*` requests whose `Origin` does not match the local server origin.
  - Rejects state-changing `/api/*` requests (`POST`, `PUT`, `PATCH`, `DELETE`) with foreign `Sec-Fetch-Site` or missing `Origin` + `X-Ducky-Request: 1`.
- **Browser security headers**: Sets `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: same-origin`, and strict `Content-Security-Policy` on HTML documents.

## Filesystem Guarantees

- **No arbitrary filesystem API**: Clients cannot read, write, list, or open arbitrary host paths; folder opening is restricted to stored workspace/problem directories.
- **Path traversal & symlink guard**: Folder names and filenames reject `..`, path separators, control characters, and Windows reserved device names. Workspace operations canonicalize paths and verify descendants remain inside the configured workspace root.
- **Non-overwriting file creation**: Source files are created using `OpenOptions::create_new(true)`. Existing user code is never silently overwritten.
- **No code execution**: Ducky never compiles or executes user source files or shell commands from API inputs.

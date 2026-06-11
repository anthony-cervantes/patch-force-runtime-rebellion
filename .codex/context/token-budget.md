# Token Budget Policy

## Default exploration budget
For normal tasks:
1. Read [repo map](./repo-map.md).
2. Run `git status --short`.
3. Use targeted `rg` queries.
4. Open only files directly relevant to the current task.
5. Stop and ask before broad scans or full-suite checks.

## Avoid by default
- dependency folders and caches
- build outputs (`target/`, `dist/`, `.trunk/`)
- generated/minified assets
- coverage and fixture-heavy directories
- lockfiles (except quick package-manager identification)

## Command policy
Prefer narrow commands:
- one package
- one command path
- one function/feature area
- one source file at a time

Ask before:
- full CI runs
- full repo test sweep
- repo-wide formatting
- dependency installation
- migration generation
- destructive file operations

## Response policy
Final responses should include:
- changed files
- verification
- risk
- next step only when necessary

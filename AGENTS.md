# D&D M8 agent notes

## Setup and verification

- Bootstrap the full contributor toolchain with `just init`; it installs Rust nightly, `cargo-nextest`, `cargo-udeps`, audit/coverage tools, pnpm dependencies, Playwright browsers, and hooks. Linux Tauri development additionally needs the packages listed in `README.md`.
- Run the desktop app with `just dev` (Tauri starts `pnpm dev`); Vite is deliberately fixed to port 1420 and fails if occupied.
- Use `just pre-commit` before a full commit. It formats both languages, then runs frontend and Rust CI checks. The non-mutating equivalent is `just ci`.
- Scope routine checks when possible: `just check-js`, `just test-js -- <vitest args>`, or `just check-rs -- -D warnings`. Rust recipes run from `src-tauri`; for a focused Rust test use `cd src-tauri && cargo nextest run -p <crate> <test-filter>`.
- `pnpm test` includes node tests plus headless Chromium client and Storybook test projects. Run `just deps-playwright` (or `pnpm exec playwright install`) before it on a new machine.

## Architecture boundaries

- `src/routes/+page.svelte` only mounts `AppShell`; application UI/state flow starts in `src/lib/components/AppShell.svelte` and `src/lib/stores/`.
- Keep all frontend-to-Rust IPC in `src/lib/api/index.ts`; views and stores must not call Tauri `invoke()` directly. Tauri command names, DTOs, and registration live in `src-tauri/src/{command,dto,lib}.rs` and must stay aligned with that API boundary.
- `src-tauri` is a Cargo workspace: the Tauri shell depends on `dnd-m8-core`, `dnd-m8-ruleset` defines the ruleset contract, and `crates/rulesets/dnd-m8-srd5-2-1` is a concrete ruleset.
- SvelteKit is built as a static SPA (`adapter-static` fallback `index.html`) for Tauri; do not introduce server-only/SSR assumptions.

## Conventions that checks enforce

- Rust crates warn on missing docs, including private items, and CI promotes warnings to errors. Follow `docs/dev/rustdoc-style.md` for non-generated Rust additions.
- `cargo +nightly fmt --all` is required for Rust formatting; use `just fmt-rs` rather than stable `cargo fmt` when matching CI.
- Storybook stories are part of the Vitest projects (`src/**/*.stories.{ts,svelte}`); update them when changing reusable UI behavior.
- Pre-commit hooks run `just ci-js` for frontend/config changes and `just ci-rs` for Rust changes; commit messages must satisfy Conventional Commits.

## Rust styling and lints

- Use descriptive intermediate variables.
- Separate logical phases with blank lines.
- Avoid dense multi-responsibility functions.
- Prefer explicit state and lifecycle operations.
- Keep nontrivial functions reasonably small.
- Crates under `src-tauri/crates/` enable `missing_docs` and `clippy::missing_docs_in_private_items`; document new non-test modules and items in the established Rustdoc style. See `docs/dev/rustdoc-style.md`.
- Do not solve lint diagnostics by adding `allow` or `expect` attributes, fix the underlying issue instead.
- Perform a visual-paragraphing review over every changed function.

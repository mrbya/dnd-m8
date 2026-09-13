#!/usr/bin/env just --justfile
set dotenv-load := true

# Output this list.
list:
    @just --list

# Installs node deps.
deps *FLAGS:
    pnpm install {{FLAGS}}

# Installs node deps with --frozen-lockfile.
deps-ci:
    @just deps --frozen-lockfile

# Installs playwright for js/ts/svelte source tests.
deps-playwright:
    pnpm exec playwright install

# Apply strict formatting to js/ts/svelte sources.
fmt-js:
    pnpm format

# Checks formatting of js/ts/svelte sources.
fmt-js-check:
    pnpm format-check

# Apply strict formatting to rust sources.
[working-directory: 'src-tauri']
fmt-rs *FLAGS:
    cargo +nightly fmt --all {{FLAGS}}

# Apply strict formatting to all sources.
fmt:
    @just fmt-js
    @just fmt-rs

# Runs svelte-check on js/ts/svelte sources.
check-js *FLAGS:
    pnpm check {{FLAGS}}

# Runs eslint check on js/ts/svelte sources.
check-js-eslint *FLAGS:
    pnpm check:eslint {{FLAGS}}

# Runs clippy on rs sources, tests, examples, while testing all features.
[working-directory: 'src-tauri']
check-rs *FLAGS:
    cargo clippy --tests --examples --all-targets --all-features --workspace {{FLAGS}}

# Runs linter checks on sources.
check:
    @just check-js
    @just check-js-eslint
    @just check-rs

# Checks for unused dependencies in js/ts/svelte sources.
unused-js:
    pnpm unused

# Checks for unused dependencies in rust sources.
[working-directory: 'src-tauri']
unused-rs:
    cargo +nightly udeps --all-targets

# Checks for unused dependencies in all sources.
unused:
    @just unused-js
    @just unused-rs

# Audits for js source vulnerabilities.
audit-js *FLAGS:
    pnpm audit --prod {{FLAGS}}

# Audits for vulnerabilities in rust sources.
[working-directory: 'src-tauri']
audit-rs *FLAGS:
    cargo audit {{FLAGS}}

# Audits sources for vulnerabilities and unused deps.
audit:
    @just audit-js
    @just audit-rs

# Runs rust unit tests.
[working-directory: 'src-tauri']
test-rs *FLAGS:
    cargo test --workspace --doc
    cargo nextest run --all-features --workspace {{FLAGS}}

# Runs frontend unit tests.
test-js *FLAGS:
    pnpm test {{FLAGS}}

# Runs whole test suite.
test:
    @just test-js
    @just test-rs

# Runs tests with a coverage report for js/ts/svelte sources.
test-cov-js:
    pnpm test:coverage

# Runs tests with a coverage report for rs sources.
[working-directory: 'src-tauri']
test-cov-rs *FLAGS:
    cargo llvm-cov nextest --all-features --workspace {{FLAGS}}

# Runs whole test suite with coverage reporting.
test-cov:
    @just test-cov-js
    @just test-cov-rs

# A thorough frontend source check ran before running commits and ci-builds.
thorough-check-js:
    @just fmt-js-check
    @just check-js
    @just check-js-eslint
    @just audit-rs
    @just unused-js

# A thorough rs backend source check ran before running commits and ci-builds.
thorough-check-rs:
    @just fmt-rs --check
    @just check-rs -- -D warnings
    @just audit-rs
    @just unused-rs

# A thorough codebase check ran before running ci-builds.
thorough-check:
    @just thorough-check-js
    @just thorough-check-rs

# Non mutating pre-commit recipe for frontend source pre commit hooks.
ci-js:
    @just thorough-check-js
    @just test-js

# Non mutating pre-commit recipe for backed source pre commit hooks.
ci-rs:
    @just thorough-check-rs
    @just test-rs

# Non mutating pre-commit recipe for ci runs.
ci:
    @just ci-js
    @just ci-rs

# Runs formating, tests and checks necessary before a commit.
pre-commit:
    @just fmt
    @just ci-js
    @just ci-rs

# Run app in development.
dev *FLAGS:
    cargo tauri dev {{FLAGS}}

# Generates icon pack from the icon.svg provided in src-tauri/icons.
icons:
    cargo tauri icon static/icon.svg

# Build app for Linux:
build:
    cargo tauri build

# Build app release setup for windows.
build-windows:
    cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

# Generate SBOM for js/ts/svelte sources.
sbom-js:
    mkdir -p sbom
    pnpm sbom --sbom-format spdx --prod > sbom/sbom-frontend.json

# Generate SBOM for rs sources.
[working-directory: 'src-tauri']
sbom-rs:
    mkdir -p ../sbom
    cargo sbom > ../sbom/sbom-backend.json

# Generates SBOM for all sources.
sbom:
    @just sbom-js
    @just sbom-rs

# Initializes the project by installing all necessary tooling. Should be run once before beginning of development.
init:
    echo # installing nightly, windows-msvc target and xwin
    rustup install nightly
    rustup target add x86_64-pc-windows-msvc
    cargo install --locked cargo-xwin

    echo # Chaching windows SDK
    cargo xwin cache xwin

    echo # Installing cargo-binstall for faster setup time
    cargo binstall -V || cargo install cargo-binstall

    echo # Installing tauri cli
    cargo tauri -V || cargo binstall tauri-cli --no-confirm

    echo # Installing test, coverage, lint, audit and other utilities
    rustup component add llvm-tools-preview
    cargo binstall cargo-llvm-cov --no-confirm
    cargo nextest -V || cargo binstall nextest --no-confirm
    cargo udeps -V || cargo binstall cargo-udeps --no-confirm
    cargo audit fix -V || cargo install cargo-audit --locked --features=fix
    cargo sbom -V || cargo binstall cargo-sbom --no-confirm
    mdbook --version || cargo binstall mdbook --no-confirm

    echo # Installing pnpm
    pnpm_major=$(pnpm --version 2>/dev/null | cut -d. -f1)
    [[ "${pnpm_major:-0}" -lt 11 ]] && npm install -g pnpm@next-11 || true

    echo # Synch node_modules and misc dependencies
    pnpm install
    just deps-playwright

    echo # Installing git pre-commit hooks
    pre-commit --version || pip install pre-commit
    pre-commit install || echo "Failed to install pre-commit hooks!" 1>&2

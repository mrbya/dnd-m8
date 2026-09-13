# D&D Mate implementation plan

This document is the living implementation roadmap for D&D Mate. It tracks the
path from the current project skeleton to a releasable, interactive character
sheet while preserving a clean, extensible ruleset architecture.

The plan should be updated as work progresses:

- Mark completed checklist items with `[x]`.
- Update milestone status in the roadmap table.
- Record important scope or architecture decisions in the decision log.
- Add newly discovered work to the relevant milestone rather than silently
  expanding the current milestone.

## Product direction

> D&D Mate is not primarily a character database. It is the player's session
> dashboard.

The application must prioritize information and actions needed during play:
current hit points, temporary hit points, active conditions, concentration,
limited resources, spell slots, attacks, and other transient state. Less
time-sensitive character information remains available, but must not crowd out
the current session state.

Development follows these principles:

1. Build vertical slices that end in visible, usable behavior.
2. Develop the Rust API and UI together so neither dictates an unsuitable
   interface to the other.
3. Treat phone portrait layouts as a first-class target, not a compressed
   desktop layout.
4. Keep the common rules vocabulary genuinely ruleset-agnostic.
5. Prefer operations such as `take_damage` and `spend_resource` over exposing
   unrestricted state setters.
6. Automate generic mechanics incrementally; descriptive/manual handling is
   acceptable for complex rules until their semantics are well understood.

## Architecture guardrails

The Tauri crate is the composition root. It may depend on the application core,
the ruleset API, and concrete ruleset implementations. The ruleset API must
never depend on a concrete ruleset.

```text
src-tauri (composition root)
├── dnd-m8-core
├── dnd-m8-ruleset
└── dnd-m8-srd5-2-1

dnd-m8-core ────────────> dnd-m8-ruleset
dnd-m8-srd5-2-1 ───────> dnd-m8-ruleset
```

The following invariants apply throughout the roadmap:

- `dnd-m8-ruleset` is the public ruleset API/SDK. It remains small,
  semver-conscious, well documented, and free of concrete ruleset dependencies.
- Concrete ruleset crates implement the ruleset API and remain independent of
  Tauri, Svelte, application persistence, and UI concerns.
- Concrete rulesets are registered at the application boundary.
- `dnd-m8-core` does not contain an enum variant for every supported ruleset.
- Saved characters identify their ruleset explicitly and version it
  unambiguously.
- Ruleset-specific character definitions remain opaque to the application core.
- Only concepts proven to be common across rulesets belong in the shared
  runtime state.
- The UI asks about ruleset capabilities rather than checking edition IDs.
- Derived values retain their provenance so the UI can explain how a result was
  calculated.
- Physical storage is accessed behind a repository abstraction and does not
  leak into the character domain model.

## Status legend

| Status | Meaning |
| --- | --- |
| Complete | Milestone acceptance criteria are satisfied. |
| In progress | This is active implementation work. |
| Planned | Scope is outlined but work has not started. |
| Blocked | Progress requires a recorded decision or external dependency. |

## Roadmap

| Milestone | Outcome | Status |
| --- | --- | --- |
| 0. Project foundation | Tauri/Svelte workspace and quality tooling | Complete |
| 1. Walking skeleton | Rust-owned data rendered through the full stack | Planned |
| 2. UI foundation | Design system and reusable components | Planned |
| 3. Character-sheet shell | Responsive session-oriented application frame | Planned |
| 4. Core character model | Ruleset-independent runtime state and operations | Planned |
| 5. Persistence | Durable local character storage behind an abstraction | Planned |
| 6. SRD 5.2.1 foundations | First representative ruleset-backed character | Planned |
| 7. Session and combat state | Useful interactive session dashboard | Planned |
| 8. Spellcasting | Prepared spells, slots, casting, and concentration | Planned |
| 9. Equipment and attacks | Inventory, equipment, attacks, and explanations | Planned |
| 10. Multi-ruleset validation | Second ruleset proves the public abstraction | Planned |
| 11. Character creation | Guided creation and editing workflow | Planned |
| 12. Polish and release | Accessible, documented, packaged application | Planned |

## Milestone 0 — Project foundation

### Goal

Establish the repository, workspace boundaries, and development quality gates.

### Completed scope

- [x] Bootstrap Tauri 2 with Svelte 5, Vite, and TypeScript.
- [x] Use pnpm as the canonical frontend package manager.
- [x] Create the Rust workspace and initial crate layout:
  - [x] `src-tauri`
  - [x] `dnd-m8-core`
  - [x] `dnd-m8-ruleset`
  - [x] `dnd-m8-srd5-2-1`
- [x] Add formatting, linting, testing, audit, unused-dependency, coverage,
  pre-commit, and SBOM tooling.
- [x] Provide repository automation through the `justfile`.

## Milestone 1 — Walking skeleton

### Goal

Prove the complete Rust → Tauri → TypeScript → Svelte path with the smallest
useful read-only character slice. The displayed data must originate in Rust;
frontend-only fixture data does not satisfy this milestone.

### Ruleset API

- [ ] Define `RulesetId` with an unambiguous, stable identifier.
- [ ] Define `RulesetMetadata`.
- [ ] Define the initial, deliberately small `Ruleset` trait.
- [ ] Define a read-only `CharacterSummary` contract.
- [ ] Add a tiny fake/test ruleset for tests and early integration work.
- [ ] Document the public types and extension boundary.

### Application core

- [ ] Define `CharacterId`.
- [ ] Add the minimal character/application state required by the slice.
- [ ] Add `CharacterService` for retrieving the sample character.
- [ ] Add focused unit tests for successful and failed retrieval.

### Tauri shell

- [ ] Wire application state during startup.
- [ ] Register the fake/test ruleset at the composition root.
- [ ] Add a thin `get_character` command.
- [ ] Define frontend-facing DTOs independently from domain types.
- [ ] Translate domain/application errors at the shell boundary.

### Frontend

- [ ] Add a typed Tauri client wrapper.
- [ ] Load a character through the Tauri command.
- [ ] Render name, level, class, current/max HP, AC, proficiency bonus, and speed.
- [ ] Represent loading, success, and error states.
- [ ] Add focused tests for the client boundary and rendered states.

### Acceptance criteria

- [ ] Launching the application displays Rust-owned sample-character data.
- [ ] No concrete SRD 5.2.1 model is required for the sample.
- [ ] The Tauri command contains orchestration only, not D&D business logic.
- [ ] The fake ruleset can be replaced without changing the frontend contract.
- [ ] All project quality gates pass.

### Explicitly deferred

- SQLite or other durable storage
- Full SRD 5.2.1 modeling
- Spellcasting and combat automation
- Production visual design
- Character creation

## Milestone 2 — UI foundation

### Goal

Establish the visual language and reusable component layer before building large
feature screens.

### Scope

- [ ] Add and configure Storybook for Svelte components.
- [ ] Define design tokens for color, typography, spacing, radii, elevation,
  borders, and motion.
- [ ] Define light and dark themes with accessible contrast.
- [ ] Establish responsive breakpoints and layout conventions.
- [ ] Select a consistent icon strategy.
- [ ] Define interaction states: default, hover, focus, active, disabled,
  loading, warning, and error.
- [ ] Implement initial reusable primitives:
  - [ ] Button and icon button
  - [ ] Card and section card
  - [ ] Stat block
  - [ ] Badge and condition badge
  - [ ] Progress/resource indicator
  - [ ] Dialog and mobile sheet
  - [ ] Empty, loading, and error states
- [ ] Add Storybook stories for meaningful visual and responsive states.
- [ ] Add component tests where behavior, accessibility, or state transitions
  justify them.
- [ ] Verify keyboard navigation, visible focus, touch targets, and screen-reader
  labels.

### Acceptance criteria

- [ ] Feature screens can be composed primarily from documented primitives.
- [ ] Components work at phone, tablet, and desktop widths.
- [ ] Light and dark themes are visually coherent and accessible.
- [ ] Storybook provides an efficient UI review loop.
- [ ] All project quality gates pass.

## Milestone 3 — Character-sheet shell

### Goal

Build a responsive, session-oriented application frame using placeholder or
sample data before introducing complex rules.

### Scope

- [ ] Define the primary information architecture.
- [ ] Implement the character header and current-state summary.
- [ ] Implement desktop navigation and layout.
- [ ] Implement mobile bottom navigation and mobile-first layout.
- [ ] Create the primary sections:
  - [ ] Overview
  - [ ] Combat
  - [ ] Spells
  - [ ] Inventory
  - [ ] Features
- [ ] Make urgent session state visually dominant.
- [ ] Preserve navigation context across responsive layout changes.
- [ ] Add representative empty, loading, error, and dense-content states.
- [ ] Test keyboard, mouse, and touch navigation.

### Acceptance criteria

- [ ] A player can reach frequently used session information within one or two
  interactions.
- [ ] Phone portrait is usable without horizontal scrolling or miniature
  desktop controls.
- [ ] Desktop uses the available space without harming scanability.
- [ ] The shell does not contain ruleset-edition checks.
- [ ] All project quality gates pass.

## Milestone 4 — Core character model

### Goal

Introduce the ruleset-independent runtime character model and safe
application-level operations.

### Scope

- [ ] Define a versioned `RulesetRef` stored with each character.
- [ ] Define an opaque ruleset-specific character-definition payload.
- [ ] Model the common runtime state proven necessary by earlier UI work:
  - [ ] Hit points and temporary hit points
  - [ ] Generic resource pools
  - [ ] Active conditions
  - [ ] Active effects and durations
  - [ ] Concentration state
  - [ ] Spell-slot state, only if it is genuinely common at this boundary
- [ ] Define application operations:
  - [ ] Take damage
  - [ ] Heal
  - [ ] Grant or replace temporary hit points
  - [ ] Spend and restore a resource
  - [ ] Apply and remove a condition
  - [ ] Apply and remove an effect
  - [ ] Start, replace, and end concentration
- [ ] Define and enforce model invariants.
- [ ] Return domain events or operation results where useful to the UI.
- [ ] Add comprehensive unit and property tests for transitions and edge cases.

### Acceptance criteria

- [ ] Runtime state transitions occur through validated operations.
- [ ] `dnd-m8-core` has no dependency on a concrete ruleset.
- [ ] Adding a ruleset does not require adding a `Character` enum variant.
- [ ] Common state is not contaminated with unproven SRD 5.2.1 assumptions.
- [ ] All project quality gates pass.

## Milestone 5 — Persistence

### Goal

Persist characters locally without coupling domain behavior to a storage
technology.

### Scope

- [ ] Finalize the persistence requirements based on Milestones 1–4.
- [ ] Define a `CharacterRepository` abstraction.
- [ ] Decide between SQLite + sqlx and a simpler file-backed implementation.
- [ ] If SQLite is selected:
  - [ ] Add sqlx with the minimum required feature set.
  - [ ] Define the initial schema and migrations.
  - [ ] Configure compile-time/offline query checking where appropriate.
- [ ] Implement create, load, update, list, and delete operations.
- [ ] Preserve ruleset identity and ruleset-data schema/version information.
- [ ] Define atomicity and recovery behavior for interrupted writes.
- [ ] Add integration tests using isolated temporary storage.
- [ ] Add useful user-facing errors for unavailable or corrupt storage.

### Acceptance criteria

- [ ] Characters survive application restarts.
- [ ] The domain model contains no direct filesystem or database calls.
- [ ] Schema migrations are deterministic and tested.
- [ ] Unsupported/missing rulesets fail safely without destroying character
  data.
- [ ] All project quality gates pass.

## Milestone 6 — SRD 5.2.1 foundations

### Goal

Implement enough of the native SRD 5.2.1 ruleset to represent and calculate a
real, representative character.

### Scope

- [ ] Establish the rules-data representation and provenance conventions.
- [ ] Model abilities, scores, and modifiers.
- [ ] Model proficiency bonus, skills, and saving throws.
- [ ] Model level and class/species/background identity.
- [ ] Model AC, movement, senses, and hit-point derivation.
- [ ] Model choices required by the representative character.
- [ ] Introduce capability metadata instead of edition checks.
- [ ] Preserve explanations/provenance for derived values.
- [ ] Populate only the SRD content needed for the representative character.
- [ ] Select a caster as the primary representative fixture so the design is
  tested against a demanding use case.
- [ ] Add rules conformance tests with traceable SRD references.

### Acceptance criteria

- [ ] The application can load and display a representative SRD 5.2.1 caster.
- [ ] Displayed derived values can explain their inputs and modifiers.
- [ ] SRD-specific concepts remain inside the ruleset implementation unless
  proven common.
- [ ] The ruleset can be used independently of the Tauri application.
- [ ] All project quality gates pass.

## Milestone 7 — Session and combat state

### Goal

Deliver the first genuinely useful alpha: a player can track the state that
changes during a session or encounter.

### Scope

- [ ] Implement damage, healing, and temporary hit-point interactions.
- [ ] Implement death-save tracking and reset behavior where provided by the
  active ruleset.
- [ ] Implement limited-use resource tracking.
- [ ] Implement active conditions and effects.
- [ ] Implement duration tracking.
- [ ] Implement concentration start, replacement, checks, and termination.
- [ ] Implement short-rest and long-rest operations through ruleset behavior.
- [ ] Surface all active, urgent state prominently in the UI.
- [ ] Add confirmation/undo behavior for costly or destructive session actions.
- [ ] Add a compact interaction history where it materially helps recovery from
  mistakes.
- [ ] Test compound transitions and edge cases.

### Acceptance criteria

- [ ] A player can run a representative combat without using paper to track
  common transient state.
- [ ] Conditions and concentration are difficult to overlook.
- [ ] Resource restoration follows ruleset behavior rather than UI assumptions.
- [ ] Invalid state transitions are rejected safely and explained clearly.
- [ ] All project quality gates pass.

## Milestone 8 — Spellcasting

### Goal

Support practical spell preparation and casting without requiring executable
implementations of every spell.

### Scope

- [ ] Model spell definitions and searchable spell metadata.
- [ ] Model known and prepared spells.
- [ ] Model spell slots and restoration.
- [ ] Model casting time, range, components, duration, concentration, and ritual
  metadata.
- [ ] Model upcasting metadata where it can be expressed generically.
- [ ] Implement casting operations that consume resources correctly.
- [ ] Replace or end concentration when required.
- [ ] Support descriptive/manual effects for spells not yet automated.
- [ ] Design fast filtering for prepared, concentration, ritual, level, and
  casting-time views.
- [ ] Add tests for preparation limits, casting, slot use, upcasting, and
  concentration interactions.

### Acceptance criteria

- [ ] A caster can prepare, find, cast, and track spells efficiently during play.
- [ ] Casting updates slots, resources, and concentration atomically.
- [ ] Lack of automated semantics never hides the spell's descriptive rules.
- [ ] All project quality gates pass.

## Milestone 9 — Equipment, attacks, and derived effects

### Goal

Connect inventory and equipped items to explainable combat calculations.

### Scope

- [ ] Model inventory entries, quantities, and equipped state.
- [ ] Model weapons, armor, shields, and other relevant equipment traits.
- [ ] Model attacks, attack bonuses, damage expressions, and damage types.
- [ ] Apply equipment-derived modifiers through the ruleset.
- [ ] Support weapon mastery through capability-provided UI and behavior.
- [ ] Explain derived values such as AC, attack bonus, and damage.
- [ ] Design quick-use attack and equipment interactions.
- [ ] Add tests for equip conflicts, derived-value recalculation, attacks, and
  stacking rules.

### Acceptance criteria

- [ ] Equipping or unequipping an item updates all dependent values correctly.
- [ ] A user can inspect why AC, attack bonus, or damage has a given value.
- [ ] Unsupported ruleset capabilities do not produce dead or misleading UI.
- [ ] All project quality gates pass.

## Milestone 10 — Multi-ruleset validation

### Goal

Prove that the ruleset API is genuinely extensible before making public semver
stability promises.

### Scope

- [ ] Implement a deliberately small second ruleset.
- [ ] Prefer a minimal SRD 5.1 adapter if a suitable external crate proves
  useful; keep that dependency isolated inside the adapter.
- [ ] Load characters from both rulesets through the same core services.
- [ ] Exercise capability-driven UI differences.
- [ ] Identify and remove leaked SRD 5.2.1 assumptions.
- [ ] Add a tiny third-party-style example ruleset or integration fixture.
- [ ] Document how to implement and register a ruleset.
- [ ] Review the public API for naming, ownership, error handling, object safety,
  serialization, and semver risk.

### Acceptance criteria

- [ ] The second ruleset requires no changes to the core character type.
- [ ] The ruleset API depends on no concrete implementation.
- [ ] Ruleset-specific UI appears through capabilities, not edition checks.
- [ ] A standalone example can consume a ruleset without Tauri or Svelte.
- [ ] The public API is ready for an explicitly documented stability policy.
- [ ] All project quality gates pass.

## Milestone 11 — Character creation and editing

### Goal

Build a guided workflow that produces valid ruleset-specific character
definitions from the already-proven domain concepts.

### Scope

- [ ] Define a ruleset-provided creation workflow and choice metadata.
- [ ] Implement class, species, background, ability, proficiency, feat,
  equipment, and spell choices as supported by the ruleset.
- [ ] Validate prerequisites and incomplete/invalid combinations.
- [ ] Preserve partially completed creation sessions.
- [ ] Support editing choices that remain legal after creation.
- [ ] Preview derived values and downstream consequences before committing.
- [ ] Design a mobile-friendly step flow with clear progress and recovery.
- [ ] Add end-to-end tests for representative simple and caster characters.

### Acceptance criteria

- [ ] A user can create, save, reopen, and use a valid representative character.
- [ ] Invalid choices are prevented or explained at the point of interaction.
- [ ] The builder consumes ruleset-provided data rather than hard-coded SRD
  edition logic.
- [ ] Editing cannot silently corrupt runtime state or persistence.
- [ ] All project quality gates pass.

## Milestone 12 — Polish and release infrastructure

### Goal

Prepare a reliable, accessible, documented release for the initially supported
platforms.

### Scope

- [ ] Perform Android device and form-factor UX testing.
- [ ] Polish keyboard and mouse workflows on desktop.
- [ ] Complete accessibility review and remediation.
- [ ] Finalize backup, restore, import, and export behavior.
- [ ] Finalize storage migrations and recovery UX.
- [ ] Add actionable crash and error reporting UX without exposing sensitive
  character data.
- [ ] Build the mdBook user/developer documentation pipeline.
- [ ] Generate and publish Rust API documentation.
- [ ] Add TypeDoc where it provides value for frontend APIs.
- [ ] Publish Storybook or equivalent visual component documentation.
- [ ] Configure release builds, packaging, signing, and CI matrices.
- [ ] Validate supported Linux, Windows, and Android targets.
- [ ] Define the later macOS/iOS enablement path without blocking the initial
  release.
- [ ] Complete license, attribution, and SRD-content review.

### Acceptance criteria

- [ ] A fresh user can install, launch, and understand the application.
- [ ] Existing character data survives supported upgrades.
- [ ] Release artifacts are reproducible through documented CI/release steps.
- [ ] User, contributor, ruleset-author, and API documentation is available.
- [ ] Supported platforms meet the agreed accessibility and UX baseline.
- [ ] All project quality gates pass.

## Global definition of done

Every implementation milestone must satisfy all applicable items below before
being marked complete:

- [ ] `just ci` passes without warnings.
- [ ] Meaningful tests cover new behavior and failure paths.
- [ ] Coverage remains above the project's agreed threshold, with a target above
  90%; low-value tests written only to inflate coverage do not count.
- [ ] No `#[allow(...)]` attributes or equivalent lint suppressions are added to
  bypass quality failures.
- [ ] Public APIs and non-obvious invariants are documented.
- [ ] Architecture boundaries and dependency direction remain intact.
- [ ] Frontend behavior includes applicable loading, empty, error, disabled,
  keyboard, touch, and responsive states.
- [ ] User-visible actions provide clear feedback and safe recovery where
  appropriate.
- [ ] Relevant documentation and this tracker are updated in the same change.

## Deferred decisions

These decisions should be made only when their milestone provides enough
evidence:

| Decision | Target milestone | Current direction |
| --- | --- | --- |
| Concrete local persistence | 5 | SQLite + sqlx is favored, but not committed. |
| Common spell-slot representation | 4/8 | Promote to core only if it proves ruleset-independent. |
| Second ruleset implementation | 10 | Minimal SRD 5.1 adapter is the leading candidate. |
| macOS and iOS support | 12+ | Preserve compatibility; do not block the first release. |

## Decision log

| Date | Decision | Rationale |
| --- | --- | --- |
| 2026-09-13 | Use vertical milestones rather than backend-first development. | Each milestone should produce visible feedback and validate the API against real UX needs. |
| 2026-09-13 | Use pnpm as the canonical frontend package manager. | It provides mature tooling, strict dependency behavior, and appropriate supply-chain controls for this stack. |
| 2026-09-13 | Treat `dnd-m8-ruleset` as an API/SDK, not a concrete-ruleset consumer. | Concrete rulesets must depend on the API, preventing a circular dependency and enabling third-party implementations. |
| 2026-09-13 | Compose concrete rulesets in `src-tauri`. | The application boundary is the correct place to register implementations without coupling the API or core to them. |
| 2026-09-13 | Bring Storybook into Milestone 2. | UI component iteration is an early project risk and benefits immediately from an isolated visual workbench. |
| 2026-09-13 | Delay full character creation until the rules and runtime models are proven. | The builder should consume established concepts instead of becoming the accidental center of the architecture. |

## Progress log

| Date | Milestone | Update |
| --- | --- | --- |
| 2026-09-13 | 0 | Project workspace and initial quality tooling bootstrapped. |
| 2026-09-13 | Planning | Living implementation plan created; Milestone 1 is next. |

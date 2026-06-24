# RFC-057 (v2) Task Breakdown / PR Plan

**Project:** omriss — Omriss Editor
**RFC:** RFC-057 — Multi-Document Workspace and Tabs (v2)
**Document type:** Task breakdown and PR sequencing plan
**Related RFC:** RFC-057 (`../../proposed/057-multi-document-workspace-and-tabs.md`)
**Lifecycle status:** inherited from RFC-057

---

## Sequencing overview

Implement as a controlled PR series. Each PR keeps the existing
single-document app behavior intact until the final shell switch is complete.

```text
PR-057-01  Workspace / Tab / TabId core model in omriss-ui
PR-057-02  DocumentIdentity + open-dedupe
PR-057-03  FocusedDraft seam (Markdown impl + fake-validator tests)
PR-057-04  App shell workspace wiring (single-document behavior preserved)
PR-057-05  Tab strip component and accessibility
PR-057-06  Tab-aware close guard
PR-057-07  Quit guard and Save All (active-only validation)
PR-057-08  Shortcuts + i18n + search/status/preview scoping
PR-057-09  Cross-platform QA and regression hardening
```

Note the v2 addition of **PR-057-03** (the draft-validity seam) as its own PR,
before any switching logic, because constraints #1/#2 hinge on it.

---

## PR-057-01 — Workspace / Tab / TabId model

**Goal.** Dioxus-free workspace state in `omriss-ui`.

**Scope.** `TabId` (monotonic, process-unique, never reused); `Tab` (no hot
draft signal inside it); `Workspace { tabs, active: Option<TabId>, next_tab_id }`;
id-first mutators; `activate_index` for Ctrl+1..9 only; `WorkspaceError`;
active invariant in every mutator; dirty helpers.

**Tests.** empty; open first/multiple; unique ids; close first/middle/active/
last; activate/close unknown id → `NoSuchTab`; dirty detection; `dirty_tabs`
order; stable id after earlier close; active invariant after every mutator.

**Acceptance.** `omriss-ui` unit tests pass without Dioxus.

---

## PR-057-02 — DocumentIdentity and open dedupe

**Goal.** Activate the existing tab when an already-open file is reopened.

**Scope.** `DocumentIdentity { SavedPath(NormalizedPath), Untitled(TabId) }`;
`NormalizedPath` (canonicalize/symlink/case-fold/Windows-prefix; safe fallback);
`Workspace::find_by_identity`. Wire identity into the open path in the state
model; UI behavior change can land with PR-057-04 if safer.

**Tests.** same canonical path dedupes; different files don't; untitled never
deduped; differing path strings that canonicalize to the same file dedupe;
canonicalization failure does not collapse distinct files.

**Acceptance.** Dedupe is Dioxus-free and covered by tests.

---

## PR-057-03 — FocusedDraft seam

**Goal.** The minimal draft-validity seam RFC-057 owns (constraints #1, #2).

**Scope.** `DraftApplicability { Applicable, Invalid { message_key }, Clean }`;
`FocusedDraft` trait; Markdown body editor implements it
(`Applicable`/`Clean`, never `Invalid`); a **fake** validator in tests to
exercise the `Invalid` branch. No JSON/TOML, no adapter dependency.

**Tests.** Markdown applicability is `Applicable`/`Clean`; fake `Invalid`
blocks apply; `apply()` only mutates when `Applicable`; source text untouched
on `Invalid`.

**Acceptance.** Seam compiles and tests pass with **no** RFC-050/RFC-053
dependency. If you needed an adapter to compile this, the seam is misplaced.

---

## PR-057-04 — App shell workspace wiring

**Goal.** Replace the root single-session assumption with active-tab binding;
single-document visuals unchanged.

**Scope.** Root owns one `Signal<Workspace>` (metadata only). `AppCtx` reads
`session`/`saved_mtime`/`selected_card` via `active()`. Hot draft stays a
focused local signal (constraint #4). Document Map + Focused Content bind to
the active tab. Open/New append in the state model. Tab strip still hidden
(behind ≥2 rule or internal switch).

**Tests.** existing single-document tests still pass; open creates active tab;
New creates active untitled tab; active session reaches the panels; **typing
does not write the workspace signal** (render-boundary test).

**Acceptance.** With one document open, the app looks/behaves as before.

---

## PR-057-05 — Tab strip UI and accessibility

**Goal.** Render tabs at ≥2 documents.

**Scope.** tab strip component; headers; active highlight; dirty marker; close
affordance; tablist ARIA; horizontal overflow.

**Accessibility.** `role="tablist"`/`role="tab"`/`aria-selected`/
`aria-controls`/`aria-labelledby`; accessible unsaved suffix; keyboard-reachable
close.

**Tests.** hidden at one tab; visible at two; dirty accessible label includes
unsaved state; keyboard navigation among tabs.

**Acceptance.** A keyboard-only user can identify, focus, activate, close tabs.

---

## PR-057-06 — Tab-aware close guard

**Goal.** Extend the unsaved guard from single-document replacement to tab
close.

**Scope.** close-tab modal targets `TabId`; reuse `UnsavedDialog`; wording
"Close without saving"; save/close workflow for dirty tabs; Cancel keeps tab;
invalid draft blocks save-and-close (via the seam).

**Edge cases.** close inactive dirty tab; close active dirty tab; close while
another modal is open; close after external file change; close untitled dirty
tab.

**Tests.** clean close removes; dirty close shows modal; cancel keeps; close-
without-saving removes; save-then-close success/failure; modal target stays
correct if tab order changes (TabId, not index).

**Acceptance.** No dirty tab can be silently closed.

---

## PR-057-07 — Quit guard and Save All

**Goal.** Workspace-level quit protection with active-only validation
(constraint #3).

**Scope.** quit request handler; dirty collection; Save All / Quit without
saving / Cancel; deterministic Save All (active draft validated first, then
dirty tabs in order); untitled → Save As; external-conflict per tab; partial
failure stops and activates the failed tab.

**Save All order.** active tab's draft validated/applied first; then dirty tabs
in tab order. **Do not** scan inactive tabs for invalid drafts.

**Tests.** no dirty → immediate quit; dirty → dialog; cancel keeps open; quit
without saving exits; Save All saves all cleanly; Save All blocks on invalid
**active** draft; Save All stops on save failure and preserves saved tabs;
untitled triggers Save As; Save As cancel cancels quit.

**Acceptance.** Quit cannot silently lose unsaved work; Save All is
deterministic and conflict-safe.

---

## PR-057-08 — Shortcuts, i18n, scoping cleanup

**Goal.** Interaction polish and catalog coverage.

**Scope.** shortcuts (Ctrl/Cmd+W close; Ctrl+Tab / Ctrl+Shift+Tab; Ctrl+1..9;
Ctrl+N new) — **re-verify collisions against `keyboard::interpret` and RFC-022
first** (constraint #10); i18n keys in English + Japanese, sorted; close/clear
search on tab switch (constraint #8); `preview_open` per-tab (constraint #7);
clear stale status on switch (constraint #9).

**Tests.** shortcuts on supported platforms; all i18n keys in both catalogs;
no stale search results after switch; preview state follows each tab; status
clears on switch.

**Acceptance.** No user-facing string hardcoded; no stale cross-tab state.

---

## PR-057-09 — Cross-platform QA and regression hardening

**Goal.** Stabilize; confirm no single-document regression.

**Required checks (project gates).**

```text
cargo fmt --check
cargo test --workspace
scripts/check-rfcs.sh
manual single-document open/edit/save regression
manual multi-tab smoke test (Linux/macOS/Windows)
```

`cargo clippy --workspace --all-targets -- -D warnings` is **recommended**, not
required, unless RFC-040/CI policy is updated (constraint #11).

**Acceptance.** RFC-057 ready to move Proposed → Implemented after release
validation.

---

## Dependency graph

```text
PR-057-01
  ├─ PR-057-02
  └─ PR-057-03
       └─ PR-057-04
            ├─ PR-057-05
            ├─ PR-057-06
            │    └─ PR-057-07
            └─ PR-057-08
                 └─ PR-057-09
```

PR-057-03 (seam) gates PR-057-04+ because switching, save, preview, and close
all call the seam.

---

## Out-of-scope backlog (future RFCs)

tab reorder by drag; session restore; multi-window; split view; cross-file
search; project/workspace files; tab overflow menu; pin tabs; close others /
close to the right; per-tab search persistence.

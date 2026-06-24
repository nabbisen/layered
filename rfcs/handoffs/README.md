# RFC Handoff Management Instructions

**Project:** omriss — Omriss Editor  
**Document type:** Repository instruction / process note  
**Primary audience:** Architect, RFC authors, Rust developers, QA engineer  
**Applies to:** RFC implementation handoffs, PR plans, and QA checklists

---

## 1. Purpose

Developer handoffs are implementation companions to RFCs. They translate an RFC decision into practical execution guidance for developers and QA.

A handoff answers:

- what should be implemented;
- where the implementation should live;
- which PR sequence is recommended;
- which tests and QA checks prove completion;
- which risks must be guarded during implementation.

A handoff is not an RFC and must not override an RFC decision.

---

## 2. Repository location

Handoffs are stored under the RFC tree because they are directly tied to RFCs:

```text
rfcs/
  proposed/
    057-multi-document-workspace-and-tabs.md

  done/
    ...

  handoffs/
    057-multi-document-workspace-and-tabs/
      implementation-handoff.md
      task-breakdown-pr-plan.md
      acceptance-qa-checklist.md
      README.md
```

Use:

```text
rfcs/handoffs/<NNN-rfc-slug>/
```

Do not use separate handoff lifecycle directories such as:

```text
rfcs/handoffs/proposed/
rfcs/handoffs/done/
```

The RFC is the lifecycle authority.

---

## 3. Status inheritance

A handoff has no independent status.

Its status is inherited from the corresponding RFC:

| RFC location / status | Handoff meaning |
|---|---|
| `rfcs/proposed/NNN-slug.md` | Handoff is proposed / implementation guidance is not yet historical |
| `rfcs/done/NNN-slug.md` | Handoff is completed / historical implementation record |
| RFC withdrawn | Handoff is withdrawn with the RFC |
| RFC superseded | Handoff is superseded with the RFC unless a newer RFC explicitly adopts it |

Do not add separate `Status: Proposed`, `Status: Done`, or similar lifecycle fields to handoff files. If a handoff needs a document metadata line, use:

```text
Related RFC: RFC-NNN
Lifecycle status: inherited from RFC-NNN
```

---

## 4. Required files for large RFCs

For large, multi-PR, cross-crate, UI-affecting, or QA-heavy RFCs, create a handoff directory with three standard files:

```text
implementation-handoff.md
```

Explains the intended implementation, architecture constraints, crate boundaries, invariants, and risks.

```text
task-breakdown-pr-plan.md
```

Splits the implementation into reviewable PRs, with dependencies and expected checks per PR.

```text
acceptance-qa-checklist.md
```

Defines developer acceptance criteria, regression checks, manual QA, accessibility checks, i18n checks, and release readiness checks.

A small RFC may omit a handoff if the RFC itself already contains enough implementation guidance.

---

## 5. Naming convention

The handoff directory uses the same number and slug as the RFC:

```text
rfcs/proposed/057-multi-document-workspace-and-tabs.md
rfcs/handoffs/057-multi-document-workspace-and-tabs/
```

Inside the directory, use stable generic filenames:

```text
implementation-handoff.md
task-breakdown-pr-plan.md
acceptance-qa-checklist.md
README.md
```

Do not include `proposed`, `done`, `draft`, or date-based status words in the handoff filename unless the file is intentionally archived outside the active handoff directory.

---

## 6. Relationship to RFC decisions

The RFC is authoritative for product and architecture decisions.

The handoff is authoritative only for execution guidance that does not conflict with the RFC.

Rules:

1. A handoff must link to its related RFC.
2. A handoff must not introduce a new product decision that is absent from or contrary to the RFC.
3. If implementation planning discovers a design conflict, update the RFC first.
4. If the RFC changes, update the handoff in the same PR or a clearly linked follow-up PR.
5. If the handoff is stale, developers must treat the RFC as the source of truth.

---

## 7. README inside each handoff directory

Each handoff directory should include a short `README.md`:

```markdown
# Handoff: RFC-NNN — Title

Related RFC: `../../proposed/NNN-slug.md` or `../../done/NNN-slug.md`

Lifecycle status: inherited from RFC-NNN.

Files:

- `implementation-handoff.md` — implementation guidance
- `task-breakdown-pr-plan.md` — recommended PR sequence
- `acceptance-qa-checklist.md` — acceptance and QA gates
```

When the RFC moves from `proposed/` to `done/`, update the relative RFC link if needed. The handoff directory itself does not move.

---

## 8. What belongs in a handoff

Include:

- implementation constraints;
- crate/module ownership;
- known architectural risks;
- regression blockers;
- PR order;
- test requirements;
- QA scenarios;
- accessibility and i18n checks;
- edge cases;
- acceptance gates.

Do not include:

- raw chat transcript;
- unresolved brainstorming;
- obsolete drafts;
- decisions that contradict the RFC;
- long historical debate unless it is required to understand an implementation risk.

---

## 9. Example: RFC-057

RFC:

```text
rfcs/proposed/057-multi-document-workspace-and-tabs.md
```

Handoff:

```text
rfcs/handoffs/057-multi-document-workspace-and-tabs/
  implementation-handoff.md
  task-breakdown-pr-plan.md
  acceptance-qa-checklist.md
  README.md
```

The handoff inherits RFC-057's status. If RFC-057 is still proposed, the handoff is proposed by inheritance. If RFC-057 is later implemented and moved to `rfcs/done/`, the handoff becomes historical by inheritance without moving directories.

---

## 10. Recommended repository check

The RFC checker may optionally verify:

- every `rfcs/handoffs/NNN-slug/` directory has a matching RFC in `rfcs/proposed/`, `rfcs/done/`, or archive location;
- large RFCs that declare `Document type: Detailed RFC design` and touch multiple crates have a handoff directory;
- handoff README links resolve;
- handoff files do not define independent lifecycle status.

This check should be advisory at first unless the project decides to make handoffs mandatory for specific RFC categories.

---

## 11. Final rule

> RFCs define decisions. Handoffs define execution. Handoffs live under `rfcs/handoffs/`, but their lifecycle status is inherited from the corresponding RFC.

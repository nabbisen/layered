# RFC-056: YAML Feasibility Spike

**Project:** omriss — Omriss Editor
**Milestone:** M12 — Structured Format Support
**Status.** Proposed
**Document type:** Detailed RFC design
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer
**Depends on:** RFC-053
**Related RFCs:** RFC-054, RFC-055

---

## 1. Summary

This RFC defines a feasibility spike for YAML support in omriss.

YAML is widely used and visually hierarchical, so it appears to fit omriss well. However, YAML is significantly more complex than JSON and TOML. Its indentation rules, anchors, aliases, tags, multiline strings, multiple documents, and parser differences make safe source-preserving editing difficult.

Therefore this RFC does **not** approve production YAML editing. It approves an investigation and, at most, a read-only or limited-safe structure view.

## 2. Motivation

YAML is common in:

- CI configuration;
- deployment files;
- Kubernetes manifests;
- documentation metadata;
- app configuration;
- local structured notes.

Users may reasonably expect omriss to navigate YAML level by level.

However, omriss must not damage user files. YAML support must be proven before editing is offered.

## 3. Goals

- Investigate whether omriss can safely build a Document Map from real-world YAML files.
- Identify YAML features that are safe, risky, or unsupported.
- Determine whether source-preserving focused editing is realistic.
- Provide a read-only YAML structure view if safe enough.
- Define acceptance gates for any future editable YAML RFC.
- Avoid misleading users into believing YAML editing is fully supported.

## 4. Non-goals

- This RFC does not approve editable YAML.
- This RFC does not implement YAML structural operations.
- This RFC does not implement schema validation.
- This RFC does not fetch remote schemas.
- This RFC does not implement Kubernetes-specific UI.
- This RFC does not normalize or format YAML.
- This RFC does not promise compatibility with every YAML parser.

## 5. Why YAML is risky

YAML can contain features that are difficult to represent safely in a simple tree editor.

Examples:

```yaml
common: &common
  image: app:latest
  replicas: 2

service_a:
  <<: *common
  replicas: 3
```

Anchors and aliases mean that what appears in one place may depend on another place.

```yaml
message: |
  This is a multiline string.
  Indentation matters here.
```

Multiline strings preserve content in ways that simple focused replacement can accidentally damage.

```yaml
---
name: first
---
name: second
```

Multiple documents in one file require a root model that is not the same as JSON/TOML.

```yaml
!!str 123
```

Tags may change value interpretation.

Because of these features, a naive YAML editor can easily change meaning while appearing to make a small edit.

## 6. Feasibility questions

The spike must answer:

1. Can we reliably build a useful Document Map for common YAML files?
2. Can we preserve source ranges for focused nodes?
3. Can we distinguish safe nodes from unsafe nodes?
4. Can we show anchors, aliases, and tags without confusing users?
5. Can we safely edit simple scalar values without changing unrelated meaning?
6. Can we preserve comments and indentation?
7. Can malformed YAML be handled gracefully?
8. Should YAML support be limited to read-only navigation?

## 7. Candidate UX

### 7.1 Read-only YAML structure view

```text
Document Map

▾ document 1
  ▾ metadata
    name
    labels
  ▾ spec
    replicas
    containers
```

Right panel:

```text
replicas

This part can be viewed here.
Safe editing for YAML is not ready yet.

[Show plain file text]
```

### 7.2 Limited safe scalar editing, only if proven

If the spike proves that simple scalar ranges can be safely replaced, the right panel may support a later experimental mode:

```text
replicas

Number
[ 3 ]

[Done]
```

But this RFC does not approve that implementation yet.

## 8. Plain labels

| YAML concept | omriss label |
|--------------|--------------|
| mapping | group |
| sequence | list |
| scalar | value |
| document | document |
| anchor | shared label |
| alias | reference |
| tag | special type |

For normal users, avoid exposing anchor/alias/tag unless necessary to explain why editing is disabled.

Example:

```text
This item uses a YAML reference, so omriss will not edit it yet.
```

## 9. Feasibility dataset

The spike should test real-world YAML examples, including:

- simple key/value config;
- nested maps;
- sequences of scalars;
- sequences of maps;
- GitHub Actions-like files;
- Kubernetes-like manifests;
- front matter-like YAML blocks if applicable;
- comments before/after keys;
- anchors and aliases;
- merge keys;
- tags;
- multiline block scalars;
- folded scalars;
- multiple documents;
- empty values;
- deeply nested files;
- malformed YAML.

No private or sensitive user files should be included in committed test fixtures unless explicitly sanitized.

## 10. Technical investigation areas

### 10.1 Parser capability

Evaluate whether available Rust YAML parsing options can provide:

- syntax validation;
- node kind;
- source ranges;
- comments;
- tags;
- anchors/aliases;
- multiple documents;
- round-trip preservation.

If source ranges or comments cannot be retained, production editing should not proceed.

### 10.2 Range safety

Determine whether focused ranges can be replaced without changing indentation or parent structure.

### 10.3 Comment preservation

YAML comments are often meaningful to users. If comments cannot be preserved reliably, editing must remain limited or read-only.

### 10.4 Indentation preservation

The spike must verify that focused edits do not alter indentation or list nesting outside the target range.

### 10.5 Ambiguous scalar types

YAML has historically had differences in scalar interpretation. The spike must define whether omriss shows raw scalar text, parsed value type, or both.

## 11. Proposed spike phases

### Phase 1 — Parser survey

- Evaluate candidate parser/editing libraries.
- Check source range support.
- Check comment preservation support.
- Check YAML version behavior.
- Document limitations.

### Phase 2 — Read-only structure prototype

- Build Document Map for selected fixture set.
- Mark unsupported/complex nodes.
- Render focused read-only summaries.
- Handle malformed files gracefully.

### Phase 3 — Preservation experiment

- Attempt safe scalar value replacement on simple YAML.
- Verify unrelated bytes remain identical.
- Verify indentation remains valid.
- Verify comments remain unchanged.
- Reject cases involving anchors, aliases, tags, block scalars, and multiple documents unless proven safe.

### Phase 4 — Decision report

The spike must produce a recommendation:

```text
A. No YAML support.
B. Read-only YAML structure view only.
C. Experimental limited scalar editing.
D. Full YAML editing roadmap after additional RFCs.
```

## 12. Safety policy

Until a later RFC accepts editable YAML:

- YAML files must not be edited through structured controls by default.
- The right panel should show read-only summaries or `Show plain file text`.
- Save must not rewrite YAML unless the user edits through an explicitly supported safe path.
- Unsupported complex nodes must be marked as view-only.

## 13. Error handling

| Cause | User-facing message |
|-------|---------------------|
| malformed YAML | “This file does not look like valid YAML.” |
| unsupported YAML feature | “This part can be viewed, but safe editing is not ready yet.” |
| anchor/alias complexity | “This item uses a YAML reference, so omriss will not edit it yet.” |
| unsafe range | “This change could not be made safely.” |

## 14. Acceptance criteria for the spike

The spike is complete when it produces:

- parser capability report;
- fixture list and test results;
- read-only Document Map prototype result or reason for rejection;
- source-range preservation findings;
- clear safe/unsafe feature matrix;
- recommendation A/B/C/D from Phase 4;
- follow-up RFC proposal if editable YAML is recommended.

## 15. YAML feature matrix template

| Feature | View in map | Focus view | Edit support | Notes |
|---------|-------------|------------|--------------|-------|
| Simple map | TBD | TBD | TBD | |
| Simple sequence | TBD | TBD | TBD | |
| Scalar string | TBD | TBD | TBD | |
| Scalar number | TBD | TBD | TBD | |
| Comments | TBD | TBD | TBD | |
| Anchors | TBD | TBD | TBD | |
| Aliases | TBD | TBD | TBD | |
| Merge keys | TBD | TBD | TBD | |
| Tags | TBD | TBD | TBD | |
| Block scalar | TBD | TBD | TBD | |
| Folded scalar | TBD | TBD | TBD | |
| Multiple documents | TBD | TBD | TBD | |

## 16. Non-acceptance criteria

The spike must **not** be considered successful if it only demonstrates that YAML can be parsed into generic values. omriss needs source-preserving, user-safe behavior. Generic parse/serialize success is insufficient.

The spike must also fail editable YAML if:

- comments cannot be preserved;
- source ranges cannot be trusted;
- block scalars are easily corrupted;
- anchors/aliases cannot be identified;
- invalid edits can pass silently;
- unrelated bytes change after focused edits.

## 17. Open questions

1. Should YAML front matter inside Markdown be handled by the Markdown adapter or a YAML adapter?
2. Should omriss support read-only YAML even if editing is rejected?
3. Should Kubernetes-specific YAML be intentionally out of scope?
4. Should YAML support be hidden behind an experimental setting during the spike?

## 18. Final decision summary

YAML is potentially useful but too risky to treat like JSON or TOML immediately. This RFC approves only a feasibility spike and possibly read-only structure view work. Production YAML editing requires a later RFC after preservation and safety have been proven.

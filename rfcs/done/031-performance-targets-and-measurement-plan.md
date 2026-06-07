<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-031: Performance Targets and Measurement Plan

**Project:** layerd — Layer EDitor  
**Milestone:** M7 — Performance and Large Document Readiness  
**Status.** Implemented (v0.10.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define measurable performance budgets and benchmarking fixtures.

## 2. Goals

- Set document size targets.
- Measure indexing, editing, rendering, and memory.
- Create regression thresholds.
- Avoid premature optimization.

## 3. Non-Goals

- No hard real-time guarantee.
- No extremely large database-like document target.
- No optimization before measurement.

## 4. Design

### Initial Budgets

| Scenario | Target |
|---|---|
| Open 10k-word Markdown | responsive enough for interactive use |
| Re-index after section commit | normally below noticeable delay for ordinary docs |
| Typing in focus editor | no full-document mutation per keystroke |
| Save | bounded by filesystem and document size |

Exact millisecond thresholds should be calibrated after M0/M1 measurements.

### Measurement Points

```text
file read time
UTF-8 decode time
index build time
focus snapshot creation time
section replacement time
re-index time
Dioxus render/update time
save write time
memory peak
```

## 5. Internal Design Notes

### Benchmark Harness

Use `criterion` or a simple stable benchmark runner. Keep performance fixtures checked in or generated deterministically.

## 6. Validation and Test Plan

- Benchmark command runs in CI optional mode.
- Performance fixtures load successfully.
- Regression thresholds documented before enforcement.

## 7. Acceptance Criteria

- Performance discussions refer to measured data.
- Input latency is protected by local edit buffer design.
- Large-document readiness has explicit fixture coverage.

## 8. Dependencies

- RFC-012
- RFC-032
- RFC-034

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.

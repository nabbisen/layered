<!-- fixture: academic-paper.md
     Purpose: realistic academic-document structure with abstract, sections,
     references, and mixed heading depths. Tests outline tree shape and path
     resolution for documents users actually author.
-->
# On Source Preservation in Structured Text Editors

## Abstract

Modern text editors that expose document structure rely on derived indexes
built from the underlying source text. When the index and source diverge,
edits may corrupt unrelated bytes. This paper formalises the *source
preservation invariant* and describes how layered enforces it through
range-bounded section operations.

## 1 Introduction

Structured editors fall into two broad categories: those that treat the
document as a tree of nodes, and those that treat it as a linear character
stream annotated with structure. The former risks divergence; the latter
sacrifices navigability.

### 1.1 Motivation

Users writing long-form Markdown documents lose context when the editor
does not expose hierarchy. A zoomed-focus model restores that context
without changing the underlying file format.

### 1.2 Contributions

This paper contributes:

- A formal definition of the source-preservation invariant.
- A taxonomy of structural edit types and their risk profiles.
- An implementation sketch using byte-range-indexed section nodes.

## 2 Background

### 2.1 Markdown as a Document Format

Markdown was designed as a lightweight authoring format with simple heading
syntax. Its heading hierarchy is implicit: a level-2 heading is subordinate
to the preceding level-1 heading by convention, not by formal grammar.

### 2.2 Editor Architecture Patterns

Previous work on structured editors includes projectional editors and
language-server-based outline views. Neither approach handles the full range
of Markdown edge cases including setext headings, fenced code blocks, and
front matter.

## 3 The Source Preservation Invariant

**Invariant.** For any edit operation *E* applied to section *S* in document
*D*, the bytes of every section *S\'* where *S\' ≠ S* and *S\' ∉ subtree(S)*
must be identical in *D* and *E(D)*.

### 3.1 Structural Exceptions

The invariant is intentionally relaxed for structural operations. A *promote*
operation changes the heading marker bytes of section *S* without touching
its body. A *move* operation relocates the full range of *S*, preserving all
bytes within that range but changing their absolute position.

### 3.2 Re-indexing after Edit

Every committed edit triggers a full re-index. If re-indexing fails, the
edit is rolled back and the canonical source is restored unchanged.

## 4 Implementation

### 4.1 Node Identity

Section nodes are identified by an ordinal-path hash computed from their
position in the heading tree, not from their byte offset. This ensures that
IDs are stable across edits to sibling sections.

### 4.2 Range Representation

Each section node stores a `ByteRange` covering its heading line and a
`ByteRange` covering its body. The *full range* covers the heading through
the end of the last descendant or the end of the document, whichever comes
first.

## 5 Evaluation

### 5.1 Benchmark Fixtures

We evaluate performance on three fixture classes: small documents (< 1 000
words), medium documents (1 000 – 10 000 words), and large documents (>
10 000 words). All benchmarks are deterministic and version-controlled.

### 5.2 Results

Re-indexing 10 000 words of Markdown completes within acceptable interactive
latency on commodity hardware. Structural moves on large documents show
linear scaling with document length.

## 6 Conclusion

The source-preservation invariant provides a useful contract for structured
Markdown editors. Enforcing it at the byte-range level decouples the
navigation model from the edit model, enabling reliable undo, deterministic
testing, and safe structural transformations.

## References

1. Gruber, J. (2004). *Markdown: A text-to-HTML conversion tool for web writers.*
2. Fowler, M. (2001). *Refactoring: Improving the design of existing code.*
3. Knuth, D. (1984). *Literate Programming.* The Computer Journal, 27(2).

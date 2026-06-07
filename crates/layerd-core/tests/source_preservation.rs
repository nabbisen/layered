//! Golden integration tests (RFC-005..008, RFC-044).
//!
//! For every fixture document and every section in it, replacing that
//! section's body must leave each byte outside the replaced range untouched,
//! and a subsequent undo must restore the original source byte-for-byte.

use layerd_core::{Document, NodeId, ReplaceSectionBody};

const FIXTURES: &[(&str, &str)] = &[
    ("nested_atx.md", include_str!("fixtures/nested_atx.md")),
    (
        "duplicate_titles.md",
        include_str!("fixtures/duplicate_titles.md"),
    ),
    ("japanese.md", include_str!("fixtures/japanese.md")),
    ("code_fences.md", include_str!("fixtures/code_fences.md")),
    (
        "skipped_levels.md",
        include_str!("fixtures/skipped_levels.md"),
    ),
    (
        "no_trailing_newline.md",
        include_str!("fixtures/no_trailing_newline.md"),
    ),
    ("setext.md", include_str!("fixtures/setext.md")),
    (
        "yaml_front_matter.md",
        include_str!("fixtures/yaml_front_matter.md"),
    ),
    (
        "toml_front_matter.md",
        include_str!("fixtures/toml_front_matter.md"),
    ),
    ("crlf.md", include_str!("fixtures/crlf.md")),
    ("html_content.md", include_str!("fixtures/html_content.md")),
    ("empty_bodies.md", include_str!("fixtures/empty_bodies.md")),
    ("no_headings.md", include_str!("fixtures/no_headings.md")),
];

/// Bodies whose replacement must not introduce new structure, so the outline
/// shape (and thus every untouched byte) can be compared mechanically.
const MARKER: &str = "REPLACED-BODY-MARKER\n";

fn node_ids(doc: &Document) -> Vec<NodeId> {
    doc.outline().iter().map(|node| node.id).collect()
}

#[test]
fn every_fixture_indexes_with_valid_invariants() {
    for (name, source) in FIXTURES {
        let doc = Document::parse((*source).to_string())
            .unwrap_or_else(|err| panic!("{name}: parse failed: {err}"));
        doc.outline()
            .validate(doc.source())
            .unwrap_or_else(|err| panic!("{name}: invariants violated: {err}"));
        assert_eq!(doc.source(), *source, "{name}: source must be verbatim");
    }
}

#[test]
fn replacing_any_body_preserves_all_unrelated_bytes() {
    for (name, source) in FIXTURES {
        let pristine = Document::parse((*source).to_string()).unwrap();
        for id in node_ids(&pristine) {
            let mut doc = Document::parse((*source).to_string()).unwrap();
            let before = doc.source().to_string();
            let result = doc
                .replace_section_body(ReplaceSectionBody {
                    node_id: id,
                    base_revision: doc.revision(),
                    new_body: MARKER.to_string(),
                })
                .unwrap_or_else(|err| panic!("{name}: edit on {id:?} failed: {err}"));

            let replaced = result.replaced_range;
            let after = doc.source();

            // Prefix bytes are untouched.
            assert_eq!(
                &after[..replaced.start],
                &before[..replaced.start],
                "{name}: prefix changed when editing {id:?}"
            );
            // Suffix bytes are untouched (shifted by the length delta).
            assert_eq!(
                &after[result.new_range.end..],
                &before[replaced.end..],
                "{name}: suffix changed when editing {id:?}"
            );
            // The replaced span now holds exactly the marker.
            assert_eq!(
                &after[result.new_range.as_range()],
                MARKER,
                "{name}: marker not stored verbatim for {id:?}"
            );
        }
    }
}

#[test]
fn undo_after_each_edit_restores_the_original_bytes() {
    for (name, source) in FIXTURES {
        let pristine = Document::parse((*source).to_string()).unwrap();
        for id in node_ids(&pristine) {
            let mut doc = Document::parse((*source).to_string()).unwrap();
            doc.replace_section_body(ReplaceSectionBody {
                node_id: id,
                base_revision: doc.revision(),
                new_body: MARKER.to_string(),
            })
            .unwrap();
            doc.undo()
                .unwrap_or_else(|err| panic!("{name}: undo on {id:?} failed: {err}"));
            assert_eq!(
                doc.source(),
                *source,
                "{name}: undo did not round-trip for {id:?}"
            );
        }
    }
}

#[test]
fn redo_after_undo_reapplies_the_exact_edit() {
    for (name, source) in FIXTURES {
        let pristine = Document::parse((*source).to_string()).unwrap();
        for id in node_ids(&pristine) {
            let mut doc = Document::parse((*source).to_string()).unwrap();
            doc.replace_section_body(ReplaceSectionBody {
                node_id: id,
                base_revision: doc.revision(),
                new_body: MARKER.to_string(),
            })
            .unwrap();
            let edited = doc.source().to_string();
            doc.undo().unwrap();
            doc.redo()
                .unwrap_or_else(|err| panic!("{name}: redo on {id:?} failed: {err}"));
            assert_eq!(
                doc.source(),
                edited,
                "{name}: redo did not restore edited source for {id:?}"
            );
        }
    }
}

#[test]
fn sequential_edits_to_every_section_then_full_unwind_round_trips() {
    for (name, source) in FIXTURES {
        let mut doc = Document::parse((*source).to_string()).unwrap();
        let mut edits = 0usize;
        loop {
            // Re-read ids each round: ranges shift after every edit.
            let ids = node_ids(&doc);
            let Some(&id) = ids.get(edits) else { break };
            doc.replace_section_body(ReplaceSectionBody {
                node_id: id,
                base_revision: doc.revision(),
                new_body: MARKER.to_string(),
            })
            .unwrap_or_else(|err| panic!("{name}: sequential edit failed: {err}"));
            edits += 1;
        }
        for _ in 0..edits {
            doc.undo().unwrap();
        }
        assert_eq!(
            doc.source(),
            *source,
            "{name}: full unwind did not restore original"
        );
        assert!(!doc.can_undo(), "{name}: history should be exhausted");
    }
}

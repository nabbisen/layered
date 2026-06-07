//! RFC-044: operation-based undo/redo — byte-exact round trips, redo
//! clearing, empty-history errors, and revision semantics.

use crate::{Document, EditError, ReplaceSectionBody};

fn doc(source: &str) -> Document {
    Document::parse(source.to_string()).expect("parse")
}

fn edit(document: &mut Document, child: usize, new_body: &str) {
    let id = document.outline().root().children[child];
    document
        .replace_section_body(ReplaceSectionBody {
            node_id: id,
            base_revision: document.revision(),
            new_body: new_body.to_string(),
        })
        .unwrap();
}

#[test]
fn undo_restores_pre_edit_source_byte_exactly() {
    let original = "# A\nbody a\n\n# B\nbody b\n";
    let mut document = doc(original);
    edit(&mut document, 0, "changed\n\n");
    assert_ne!(document.source(), original);
    document.undo().unwrap();
    assert_eq!(document.source(), original);
}

#[test]
fn redo_restores_post_edit_source_byte_exactly() {
    let mut document = doc("# A\nbody\n");
    edit(&mut document, 0, "changed\n");
    let edited = document.source().to_string();
    document.undo().unwrap();
    document.redo().unwrap();
    assert_eq!(document.source(), edited);
}

#[test]
fn multiple_edits_unwind_in_reverse_order() {
    let original = "# A\none\n# B\ntwo\n";
    let mut document = doc(original);
    edit(&mut document, 0, "ONE\n");
    edit(&mut document, 1, "TWO\n");
    assert_eq!(document.source(), "# A\nONE\n# B\nTWO\n");
    document.undo().unwrap();
    assert_eq!(document.source(), "# A\nONE\n# B\ntwo\n");
    document.undo().unwrap();
    assert_eq!(document.source(), original);
}

#[test]
fn new_edit_after_undo_clears_redo_stack() {
    let mut document = doc("# A\nbody\n");
    edit(&mut document, 0, "first\n");
    document.undo().unwrap();
    assert!(document.can_redo());
    edit(&mut document, 0, "second\n");
    assert!(!document.can_redo());
    assert_eq!(document.redo().unwrap_err(), EditError::NothingToRedo);
}

#[test]
fn empty_history_returns_errors_not_panics() {
    let mut document = doc("# A\nbody\n");
    assert!(!document.can_undo());
    assert!(!document.can_redo());
    assert_eq!(document.undo().unwrap_err(), EditError::NothingToUndo);
    assert_eq!(document.redo().unwrap_err(), EditError::NothingToRedo);
}

#[test]
fn undo_and_redo_produce_fresh_revisions() {
    let mut document = doc("# A\nbody\n");
    edit(&mut document, 0, "changed\n"); // rev 1
    let undo_result = document.undo().unwrap(); // rev 2
    assert_eq!(undo_result.new_revision, document.revision());
    let redo_result = document.redo().unwrap(); // rev 3
    assert!(redo_result.new_revision > undo_result.new_revision);
}

#[test]
fn undo_across_reindex_keeps_unrelated_sections_intact() {
    let original = "# A\nplain\n\n# B\nuntouched\n";
    let mut document = doc(original);
    // This edit introduces a new heading, changing the outline shape.
    edit(&mut document, 0, "intro\n\n## Inserted\nnew\n\n");
    document.undo().unwrap();
    assert_eq!(document.source(), original);
    let b = document.outline().root().children[1];
    assert_eq!(document.section_body(b).unwrap(), "untouched\n");
}

#[test]
fn history_capacity_drops_oldest_entries() {
    let mut document = doc("# A\n0\n");
    for i in 1..=(crate::DEFAULT_HISTORY_CAPACITY + 10) {
        edit(&mut document, 0, &format!("{i}\n"));
    }
    let mut undone = 0;
    while document.undo().is_ok() {
        undone += 1;
    }
    assert_eq!(undone, crate::DEFAULT_HISTORY_CAPACITY);
}

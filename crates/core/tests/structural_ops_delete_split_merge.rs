//! Delete, split, and merge tests (RFC-025).

use omriss::{Document, HeadingLevel, StructuralEditError};

fn doc(md: &str) -> Document {
    Document::parse(md.to_string()).unwrap()
}

// ── Delete ─────────────────────────────────────────────────────────────────

#[test]
fn delete_removes_full_range() {
    let src = "# A\nbody A\n\n# B\nbody B\n";
    let mut d = doc(src);
    let b = *d.outline().root().children.last().unwrap();
    d.delete_section(b, d.revision()).unwrap();
    assert_eq!(d.source(), "# A\nbody A\n\n");
}

#[test]
fn delete_with_children() {
    let src = "# A\n\n## A.1\ntext\n\n## A.2\ntext\n\n# B\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    d.delete_section(a, d.revision()).unwrap();
    assert_eq!(d.source(), "# B\n");
}

#[test]
fn delete_root_rejected() {
    let mut d = doc("# A\n");
    let root = d.outline().root_id();
    assert_eq!(
        d.delete_section(root, d.revision()),
        Err(StructuralEditError::CannotDeleteRoot)
    );
}

#[test]
fn delete_undo_round_trip() {
    let src = "# A\nbody\n\n# B\nkeep\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    d.delete_section(a, d.revision()).unwrap();
    d.undo().unwrap();
    assert_eq!(d.source(), src);
}

// ── Split ──────────────────────────────────────────────────────────────────

#[test]
fn split_at_body_end_appends_child() {
    let src = "# A\nbody text\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    let body_len = d.outline().node(a).unwrap().body_range.len();
    d.split_section(a, body_len, "Child", HeadingLevel::H2, d.revision())
        .unwrap();
    assert!(d.source().contains("## Child\n"), "{:?}", d.source());
    assert!(
        d.source().starts_with("# A\nbody text\n"),
        "{:?}",
        d.source()
    );
}

#[test]
fn split_at_zero_prepends_child() {
    let src = "# A\nbody\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    d.split_section(a, 0, "New", HeadingLevel::H2, d.revision())
        .unwrap();
    assert!(d.source().contains("## New\n"), "{:?}", d.source());
}

#[test]
fn split_invalid_offset_rejected() {
    let src = "# A\nbody\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    assert!(
        d.split_section(a, 9999, "X", HeadingLevel::H2, d.revision())
            .is_err()
    );
}

// ── Merge ──────────────────────────────────────────────────────────────────

#[test]
fn merge_removes_heading_of_second_section() {
    let src = "# A\nbody A\n\n# B\nbody B\n";
    let mut d = doc(src);
    let b = *d.outline().root().children.last().unwrap();
    d.merge_with_prev_sibling(b, d.revision()).unwrap();
    assert!(!d.source().contains("# B\n"), "{:?}", d.source());
    assert!(d.source().contains("body A"), "{:?}", d.source());
    assert!(d.source().contains("body B"), "{:?}", d.source());
}

#[test]
fn merge_first_sibling_rejected() {
    let src = "# A\nbody\n\n# B\nbody\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    assert_eq!(
        d.merge_with_prev_sibling(a, d.revision()),
        Err(StructuralEditError::NoAdjacentSibling)
    );
}

#[test]
fn merge_undo_round_trip() {
    let src = "# A\nbody A\n\n# B\nbody B\n";
    let mut d = doc(src);
    let b = *d.outline().root().children.last().unwrap();
    d.merge_with_prev_sibling(b, d.revision()).unwrap();
    d.undo().unwrap();
    assert_eq!(d.source(), src);
}

//! Structural operation tests: golden byte-preservation, error cases, and
//! undo round-trips for all M5 operations.

use layered_core::{Document, HeadingLevel, MoveTarget, StructuralEditError};

fn doc(md: &str) -> Document {
    Document::parse(md.to_string()).unwrap()
}

fn node_ids(d: &Document) -> Vec<layered_core::NodeId> {
    d.outline().iter().map(|n| n.id).collect()
}

// ── Promote / Demote (RFC-023) ─────────────────────────────────────────────

#[test]
fn promote_h2_to_h1() {
    let src = "# A\n\n## B\nbody\n";
    let mut d = doc(src);
    let b = node_ids(&d)[2]; // root, A, B
    d.promote_section(b, d.revision()).unwrap();
    assert!(d.source().contains("# B\n"), "got: {:?}", d.source());
    assert!(d.source().starts_with("# A\n"), "prefix changed");
}

#[test]
fn demote_h1_to_h2() {
    let src = "# A\nbody\n";
    let mut d = doc(src);
    let a = node_ids(&d)[1];
    d.demote_section(a, d.revision()).unwrap();
    assert_eq!(d.source(), "## A\nbody\n");
}

#[test]
fn promote_h1_rejected() {
    let mut d = doc("# A\n");
    let a = node_ids(&d)[1];
    assert_eq!(
        d.promote_section(a, d.revision()),
        Err(StructuralEditError::InvalidLevel)
    );
}

#[test]
fn demote_h6_rejected() {
    let mut d = doc("###### A\n");
    let a = node_ids(&d)[1];
    assert_eq!(
        d.demote_section(a, d.revision()),
        Err(StructuralEditError::InvalidLevel)
    );
}

#[test]
fn promote_preserves_unrelated_bytes() {
    let src = "# Root\n\n## Target\nbody\n\n## Sibling\nkeep\n";
    let mut d = doc(src);
    let target = node_ids(&d)[2];
    let old = d.source().to_string();
    d.promote_section(target, d.revision()).unwrap();
    // Sibling bytes after Target's full range must be unchanged.
    let new = d.source();
    assert!(new.ends_with("## Sibling\nkeep\n"), "suffix: {:?}", new);
    assert_ne!(new, old);
}

#[test]
fn promote_undo_round_trip() {
    let src = "# Root\n\n## B\nbody\n";
    let mut d = doc(src);
    let b = node_ids(&d)[2];
    d.promote_section(b, d.revision()).unwrap();
    d.undo().unwrap();
    assert_eq!(d.source(), src);
}

// ── Delete (RFC-025) ───────────────────────────────────────────────────────

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

// ── Split (RFC-025) ────────────────────────────────────────────────────────

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

// ── Merge (RFC-025) ────────────────────────────────────────────────────────

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

// ── Move (RFC-024) ─────────────────────────────────────────────────────────

#[test]
fn move_before_sibling() {
    let src = "# A\n\n# B\n\n# C\n";
    let mut d = doc(src);
    let ids = node_ids(&d);
    let c = ids[3]; // root=0, A=1, B=2, C=3
    let a = ids[1];
    d.move_section(c, MoveTarget::Before(a), d.revision())
        .unwrap();
    let new = d.source();
    let pos_c = new.find("# C\n").unwrap();
    let pos_a = new.find("# A\n").unwrap();
    assert!(pos_c < pos_a, "C should appear before A");
}

#[test]
fn move_after_sibling() {
    let src = "# A\n\n# B\n\n# C\n";
    let mut d = doc(src);
    let ids = node_ids(&d);
    let a = ids[1];
    let c = ids[3];
    d.move_section(a, MoveTarget::After(c), d.revision())
        .unwrap();
    let new = d.source();
    let pos_a = new.find("# A\n").unwrap();
    let pos_c = new.find("# C\n").unwrap();
    assert!(pos_c < pos_a, "A should appear after C");
}

#[test]
fn move_into_descendant_rejected() {
    let src = "# A\n\n## A.1\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    let a1 = d.outline().node(a).unwrap().children[0];
    assert_eq!(
        d.move_section(a, MoveTarget::Before(a1), d.revision()),
        Err(StructuralEditError::CannotMoveIntoDescendant)
    );
}

#[test]
fn move_self_rejected() {
    let src = "# A\n\n# B\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    assert_eq!(
        d.move_section(a, MoveTarget::Before(a), d.revision()),
        Err(StructuralEditError::CannotMoveSelf)
    );
}

#[test]
fn move_preserves_subtree_bytes() {
    let src = "# A\n\n## A.1\ntext\n\n# B\n";
    let mut d = doc(src);
    let a = d.outline().root().children[0];
    let b = *d.outline().root().children.last().unwrap();
    d.move_section(a, MoveTarget::After(b), d.revision())
        .unwrap();
    let new = d.source();
    // B comes before A now.
    assert!(new.starts_with("# B\n"), "{:?}", new);
    // The full subtree of A (including A.1) is present after B.
    assert!(new.contains("## A.1\ntext\n"), "{:?}", new);
}

#[test]
fn move_undo_round_trip() {
    let src = "# A\n\n# B\n\n# C\n";
    let mut d = doc(src);
    let ids = node_ids(&d);
    let a = ids[1];
    let c = ids[3];
    d.move_section(a, MoveTarget::After(c), d.revision())
        .unwrap();
    d.undo().unwrap();
    assert_eq!(d.source(), src);
}

// ── Revision guard (RFC-026) ───────────────────────────────────────────────

#[test]
fn structural_revision_mismatch_rejected() {
    let mut d = doc("# A\n");
    let a = d.outline().root().children[0];
    let stale_rev = d.revision();
    // Apply another edit to bump the revision.
    d.replace_section_body(layered_core::ReplaceSectionBody {
        node_id: a,
        base_revision: d.revision(),
        new_body: "changed\n".into(),
    })
    .unwrap();
    // Structural op against stale revision must fail.
    assert!(matches!(
        d.demote_section(a, stale_rev),
        Err(StructuralEditError::RevisionMismatch { .. })
    ));
}

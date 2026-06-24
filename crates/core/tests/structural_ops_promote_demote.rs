//! Promote / demote tests (RFC-023).

use omriss::{Document, StructuralEditError};

fn doc(md: &str) -> Document {
    Document::parse(md.to_string()).unwrap()
}

fn node_ids(d: &Document) -> Vec<omriss::NodeId> {
    d.outline().iter().map(|n| n.id).collect()
}

#[test]
fn promote_h2_to_h1() {
    let src = "# A\n\n## B\nbody\n";
    let mut d = doc(src);
    let b = node_ids(&d)[2];
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

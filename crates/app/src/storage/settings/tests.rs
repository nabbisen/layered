use super::*;

fn with_recents(paths: &[&str]) -> AppSettings {
    let mut s = AppSettings::default();
    // push_recent inserts at front, so push in reverse to get the given order.
    for p in paths.iter().rev() {
        s.push_recent(p);
    }
    s
}

#[test]
fn push_recent_inserts_at_front() {
    let mut s = AppSettings::default();
    s.push_recent("/a.md");
    s.push_recent("/b.md");
    assert_eq!(s.recent_files, vec!["/b.md", "/a.md"]);
}

#[test]
fn push_recent_deduplicates_and_promotes_to_front() {
    let mut s = with_recents(&["/a.md", "/b.md", "/c.md"]);
    // Re-opening an existing file moves it to the front without duplicating.
    s.push_recent("/c.md");
    assert_eq!(s.recent_files, vec!["/c.md", "/a.md", "/b.md"]);
    assert_eq!(s.recent_files.iter().filter(|p| *p == "/c.md").count(), 1);
}

#[test]
fn push_recent_caps_at_max() {
    let mut s = AppSettings::default();
    for i in 0..(MAX_RECENT + 5) {
        s.push_recent(&format!("/file{i}.md"));
    }
    assert_eq!(s.recent_files.len(), MAX_RECENT);
    // The most recently pushed survives; the oldest are dropped.
    assert_eq!(s.recent_files[0], format!("/file{}.md", MAX_RECENT + 4));
}

#[test]
fn remove_recent_drops_only_the_named_path() {
    let mut s = with_recents(&["/a.md", "/b.md", "/c.md"]);
    s.remove_recent("/b.md");
    assert_eq!(s.recent_files, vec!["/a.md", "/c.md"]);
}

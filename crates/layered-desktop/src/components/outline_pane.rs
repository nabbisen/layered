//! Left-panel outline powered by `dioxus-swdir-tree`'s `ItemTreeView`
//! (RFC-011, RFC-014). The generic `ItemTree<String>` holds expand/collapse
//! and selection state; `set_tree` is called on every session change so the
//! widget's key-based diffing preserves expansion state across body edits.

use dioxus::prelude::*;
use dioxus_swdir_tree::item_tree::node::ItemNode;
use dioxus_swdir_tree::item_tree::node::NodeId as SwNodeId;
use dioxus_swdir_tree::{ItemTree, ItemTreeEvent, ItemTreeView};
use layered_ui::i18n::{Locale, t};
use layered_ui::{EditorSession, OutlineNode, ViewMode};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Convert `OutlineNode` (from `layered-ui`) into `ItemNode<String>` for
/// `ItemTree`. The root is included so the widget always has a single root
/// node; its label becomes the i18n "breadcrumb.root" key at render time.
fn to_item_node(node: &OutlineNode) -> ItemNode<String> {
    let id = SwNodeId(node.id);
    let children: Vec<ItemNode<String>> = node.children.iter().map(to_item_node).collect();
    if children.is_empty() {
        ItemNode::leaf(id, node.title.clone())
    } else {
        ItemNode::branch(id, node.title.clone(), children)
    }
}

// ── component ─────────────────────────────────────────────────────────────────

#[component]
pub fn OutlinePane(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
) -> Element {
    let lang = *locale.read();

    // Local ItemTree signal — lives inside this component.
    let mut item_tree = use_signal(|| ItemTree::new().with_display(|s: &String| s.clone()));

    // Sync ItemTree whenever the session (and therefore the outline) changes.
    // `set_tree` diffs by NodeId, so expansion state survives pure body edits.
    use_effect(move || {
        let root_node = session.read().outline_nodes();
        item_tree.write().set_tree(to_item_node(&root_node));
    });

    // Event handler: route tree events back into ItemTree and the editor session.
    let on_event = move |ev: ItemTreeEvent| match ev {
        ItemTreeEvent::Toggled(id) => {
            item_tree.write().on_toggled(id);
        }
        ItemTreeEvent::Selected(id, mode) => {
            item_tree.write().on_selected(id, mode);
            // Navigate the editor to the selected section.
            let layered_id = layered_core::NodeId(id.0);
            let _ = session.write().focus(layered_id);
            let body = session
                .read()
                .current_snapshot()
                .map(|s| s.body)
                .unwrap_or_default();
            draft.set(body);
        }
        ItemTreeEvent::Drag(_) => {} // drag-and-drop not enabled
    };

    rsx! {
        aside {
            class: "outline-pane",
            "aria-label": t(lang, "aria.outline"),
            h2 { {t(lang, "outline.title")} }

            ItemTreeView { tree: item_tree, on_event }

            if let ViewMode::Focus(_) = session.read().view_mode() {
                button {
                    class: "outline-up",
                    onclick: move |_| {
                        // Commit draft before navigating away.
                        let snap = session.read().current_snapshot();
                        if let Some(snapshot) = snap {
                            let current_draft = draft.read().clone();
                            if current_draft != snapshot.body {
                                let _ = session
                                    .write()
                                    .commit_focused_body(&snapshot, current_draft);
                            }
                        }
                        session.write().zoom_out();
                        let body = session
                            .read()
                            .current_snapshot()
                            .map(|s| s.body)
                            .unwrap_or_default();
                        draft.set(body);
                    },
                    {t(lang, "nav.up")}
                }
            }
        }
    }
}

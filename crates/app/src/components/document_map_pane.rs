//! Document Map panel — the single structure-organization surface (RFC-049).
//!
//! All structural editing actions (move, promote/demote, join, delete, add)
//! live here. The right-side `FocusedContentPane` contains none of them.
//!
//! ## Reactive design
//!
//! `session` is subscribed exactly once, inside a `use_effect`. The effect
//! pushes two derived local signals: `item_tree` (for `ItemTreeView`) and
//! `map_root_sig` (the `DocumentMapNode` tree for row-menu rendering).
//! The component render body reads only local signals, so a `session` change
//! does not create a direct render-time subscription that would fight the
//! effect and cause a re-render loop.

use dioxus::prelude::*;
use dioxus_swdir_tree::item_tree::node::ItemNode;
use dioxus_swdir_tree::item_tree::node::NodeId as SwNodeId;
use dioxus_swdir_tree::{ItemTree, ItemTreeEvent, SelectionMode};
use omriss_ui::i18n::{Locale, t};
use omriss_ui::{DocumentMapNode, EditorSession, MapCapability, ViewMode, node_id_from_raw};

// ── helpers ───────────────────────────────────────────────────────────────────

fn to_item_node(n: &DocumentMapNode) -> ItemNode<String> {
    let id = SwNodeId(n.id);
    let children: Vec<ItemNode<String>> = n.children.iter().map(to_item_node).collect();
    if children.is_empty() {
        ItemNode::leaf(id, n.title.clone())
    } else {
        ItemNode::branch(id, n.title.clone(), children)
    }
}

fn commit_draft_if_dirty(session: &mut Signal<EditorSession>, draft: &mut Signal<String>) {
    let snap = session.read().current_snapshot();
    let Some(s) = snap else { return };
    let d = draft.read().clone();
    if d != s.body {
        let _ = session.write().commit_focused_body(&s, d);
    }
}

fn sync_draft(session: &Signal<EditorSession>, draft: &mut Signal<String>) {
    let body = session
        .read()
        .current_snapshot()
        .map(|s| s.body)
        .unwrap_or_default();
    draft.set(body);
}

// ── component ─────────────────────────────────────────────────────────────────

#[component]
pub fn DocumentMapPane(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
    status: Signal<String>,
) -> Element {
    let lang = *locale.read();

    // Local signals — component body reads only these, never `session` directly.
    let mut item_tree = use_signal(|| ItemTree::new().with_display(|s: &String| s.clone()));
    let mut map_root_sig: Signal<Option<DocumentMapNode>> = use_signal(|| None);
    let mut menu_open_for: Signal<Option<u64>> = use_signal(|| None);

    // `session` is subscribed here only. Writing local signals inside this
    // effect is safe: Dioxus 0.7 tracks reactive deps by what is *read*
    // during the effect, not what is written. `item_tree` and `map_root_sig`
    // are therefore not deps of this effect and do not re-trigger it.
    use_effect(move || {
        let root_node = session.read().document_map_nodes();
        let view = session.read().view_mode();

        // Check whether the focused node is new BEFORE set_tree adds it.
        // `is_expanded` returns `None` for ids not yet in the tree.
        // After set_tree all nodes exist, so the only reliable "new node"
        // signal is to query before the tree update.
        let focused_is_new = if let ViewMode::Focus(focused_id) = view {
            item_tree
                .read()
                .is_expanded(SwNodeId(focused_id.0))
                .is_none()
        } else {
            false
        };

        item_tree.write().set_tree(to_item_node(&root_node));

        // Expand ancestors of a newly created focused node so it is visible.
        // For existing nodes we leave the user's expand/collapse state alone.
        if focused_is_new {
            if let ViewMode::Focus(focused_id) = view {
                let focused_sw = SwNodeId(focused_id.0);
                let mut ancestors: Vec<SwNodeId> = Vec::new();
                collect_ancestors(&root_node, focused_sw, &mut ancestors);
                // ancestors is bottom-up (child before parent); reverse to
                // expand outermost first so each level becomes visible.
                ancestors.reverse();
                for id in ancestors {
                    if item_tree.read().is_expanded(id) == Some(false) {
                        item_tree.write().on_toggled(id);
                    }
                }
                // Sync the tree's visual selection to the new node so it is
                // highlighted in the left panel, not its parent.
                item_tree
                    .write()
                    .on_selected(focused_sw, SelectionMode::Replace);
            }
        }

        map_root_sig.set(Some(root_node));
    });

    let mut on_event = move |ev: ItemTreeEvent| match ev {
        ItemTreeEvent::Toggled(id) => {
            item_tree.write().on_toggled(id);
        }
        ItemTreeEvent::Selected(id, mode) => {
            item_tree.write().on_selected(id, mode);
            menu_open_for.set(None);
            let section_id = node_id_from_raw(id.0);
            commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
            let _ = session.write().focus(section_id);
            sync_draft(&session, &mut draft.clone());
        }
        ItemTreeEvent::Drag(_) => {}
    };

    let close_menu = move |_: Event<MouseData>| {
        menu_open_for.set(None);
    };

    // Read local signals only — no session subscription here.
    let map_root = map_root_sig.read();
    // The document root's raw id — we skip action buttons for this row since
    // the top-level "+ Add section" button already covers adding H1 sections.
    let doc_root_raw_id: u64 = map_root.as_ref().map(|r| r.id).unwrap_or(u64::MAX);
    let is_empty = map_root
        .as_ref()
        .map(|r| r.children.is_empty())
        .unwrap_or(true);
    let in_focus = matches!(session.read().view_mode(), ViewMode::Focus(_));

    rsx! {
        aside {
            class: "document-map-pane",
            "aria-label": t(lang, "aria.document_map"),
            onclick: close_menu,

            h2 { class: "document-map-title", {t(lang, "document_map.title")} }

            button {
                class: "document-map-add-top",
                title: t(lang, "document_map.add_top_level"),
                onclick: move |ev| {
                    ev.stop_propagation();
                    status.clone().set("struct.split.pending".into());
                },
                "+ "
                {t(lang, "document_map.add_top_level")}
            }

            if is_empty {
                p { class: "document-map-empty", {t(lang, "document_map.empty")} }
                p { class: "document-map-hint", {t(lang, "document_map.no_headings_hint")} }
            } else {
                // Render rows directly from item_tree so each row contains
                // both the tree content and the action buttons in one element.
                // This eliminates the parallel-column alignment problem.
                div {
                    class: "document-map-tree",
                    tabindex: "0",
                    onkeydown: move |evt| {
                        use dioxus_swdir_tree::TreeKey;
                        let tree_key = match evt.key() {
                            Key::ArrowUp => TreeKey::Up,
                            Key::ArrowDown => TreeKey::Down,
                            Key::ArrowLeft => TreeKey::Left,
                            Key::ArrowRight => TreeKey::Right,
                            Key::Enter => TreeKey::Enter,
                            Key::Home => TreeKey::Home,
                            Key::End => TreeKey::End,
                            Key::Escape => TreeKey::Escape,
                            Key::Character(ref ch) if ch == " " => TreeKey::Space,
                            _ => return,
                        };
                        let mods = dioxus_swdir_tree::Modifiers {
                            shift: evt.modifiers().shift(),
                            ctrl: evt.modifiers().ctrl(),
                        };
                        if let Some(ev) = item_tree.read().handle_key(tree_key, mods) {
                            evt.prevent_default();
                            on_event(ev);
                        }
                    },
                    {item_tree.read().visible_rows().into_iter().map(|row| {
                        let node_id = node_id_from_raw(row.id.0);
                        let raw_id = row.id.0;
                        let indent_px = row.depth * 16;
                        let is_root_row = raw_id == doc_root_raw_id;
                        let caret = if row.has_children {
                            if row.is_expanded { "▾" } else { "▸" }
                        } else { " " };
                        let mut row_class = "dx-swdir-row".to_string();
                        if row.is_selected { row_class.push_str(" dx-swdir-row--selected"); }
                        rsx! {
                            div {
                                key: "{raw_id}",
                                class: "{row_class}",
                                style: "padding-left: {indent_px}px;",
                                // Caret toggles expand/collapse
                                span {
                                    class: "dx-swdir-caret",
                                    onclick: move |ev| {
                                        ev.stop_propagation();
                                        item_tree.write().on_toggled(SwNodeId(raw_id));
                                    },
                                    "{caret}"
                                }
                                // Icon
                                span { class: "dx-swdir-icon" }
                                // Label — clicking selects
                                span {
                                    class: "dx-swdir-label",
                                    style: "flex: 1; overflow: hidden; text-overflow: ellipsis;",
                                    onclick: move |_| {
                                        item_tree.write().on_selected(SwNodeId(raw_id), SelectionMode::Replace);
                                        menu_open_for.set(None);
                                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                                        let _ = session.write().focus(node_id);
                                        sync_draft(&session, &mut draft.clone());
                                    },
                                    "{row.label}"
                                }
                                // Action buttons — only for non-root rows
                                if !is_root_row {
                                    button {
                                        class: "row-add-btn",
                                        title: "Add section inside",
                                        "aria-label": "Add section inside",
                                        onmousedown: move |ev| ev.prevent_default(),
                                        onclick: move |ev| {
                                            ev.stop_propagation();
                                            commit_draft_if_dirty(
                                                &mut session.clone(),
                                                &mut draft.clone(),
                                            );
                                            let _ = session.write().focus(node_id);
                                            sync_draft(&session, &mut draft.clone());
                                            item_tree
                                                .write()
                                                .on_selected(SwNodeId(raw_id), SelectionMode::Replace);
                                            status.clone().set("struct.split.pending".into());
                                        },
                                        "+"
                                    }
                                    button {
                                        class: "row-menu-btn",
                                        title: "Section actions",
                                        "aria-label": "Section actions",
                                        onmousedown: move |ev| ev.prevent_default(),
                                        onclick: move |ev| {
                                            ev.stop_propagation();
                                            commit_draft_if_dirty(
                                                &mut session.clone(),
                                                &mut draft.clone(),
                                            );
                                            let _ = session.write().focus(node_id);
                                            sync_draft(&session, &mut draft.clone());
                                            let cur = *menu_open_for.read();
                                            menu_open_for.set(
                                                if cur == Some(raw_id) { None } else { Some(raw_id) },
                                            );
                                        },
                                        "⋯"
                                    }
                                }
                            }
                        }
                    })}
                }
            }

            // NodeRowMenu is rendered outside the overlay — it is positioned
            // absolute relative to the document-map-pane aside, with a right
            // offset so it appears next to the ⋯ button column.
            if let (Some(menu_id), Some(root)) = (*menu_open_for.read(), map_root.as_ref()) {
                if let Some(node) = find_node(root, menu_id) {
                    NodeRowMenu {
                        node: node.clone(),
                        session,
                        locale,
                        draft,
                        status,
                        menu_open_for,
                    }
                }
            }

            if in_focus {
                button {
                    class: "document-map-up",
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        session.write().zoom_out();
                        sync_draft(&session, &mut draft.clone());
                    },
                    {t(lang, "nav.up")}
                }
            }
        }
    }
}

// ── Node row menu ─────────────────────────────────────────────────────────────

#[component]
fn NodeRowMenu(
    node: DocumentMapNode,
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
    status: Signal<String>,
    mut menu_open_for: Signal<Option<u64>>,
) -> Element {
    let lang = *locale.read();
    let caps = node.capabilities.clone();
    let node_id = node_id_from_raw(node.id);

    rsx! {
        div {
            class: "row-menu",
            role: "menu",
            "aria-label": "Section actions",
            onclick: move |ev| ev.stop_propagation(),

            if !caps.can_move_up.is_hidden() {
                button {
                    class: "row-menu-item",
                    role: "menuitem",
                    disabled: !caps.can_move_up.is_allowed(),
                    title: disabled_title(lang, &caps.can_move_up),
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        let _ = session.write().focus(node_id);
                        if session.write().move_focused_up().is_ok() {
                            sync_draft(&session, &mut draft.clone());
                        }
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.move_up")}
                }
            }
            if !caps.can_move_down.is_hidden() {
                button {
                    class: "row-menu-item",
                    role: "menuitem",
                    disabled: !caps.can_move_down.is_allowed(),
                    title: disabled_title(lang, &caps.can_move_down),
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        let _ = session.write().focus(node_id);
                        if session.write().move_focused_down().is_ok() {
                            sync_draft(&session, &mut draft.clone());
                        }
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.move_down")}
                }
            }
            if !caps.can_move_inside_previous.is_hidden() {
                button {
                    class: "row-menu-item",
                    role: "menuitem",
                    disabled: !caps.can_move_inside_previous.is_allowed(),
                    title: disabled_title(lang, &caps.can_move_inside_previous),
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        let _ = session.write().focus(node_id);
                        if session.write().demote_focused().is_ok() {
                            sync_draft(&session, &mut draft.clone());
                        }
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.move_inside_previous")}
                }
            }
            if !caps.can_move_out_one_level.is_hidden() {
                button {
                    class: "row-menu-item",
                    role: "menuitem",
                    disabled: !caps.can_move_out_one_level.is_allowed(),
                    title: disabled_title(lang, &caps.can_move_out_one_level),
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        let _ = session.write().focus(node_id);
                        if session.write().promote_focused().is_ok() {
                            sync_draft(&session, &mut draft.clone());
                        }
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.move_out_one_level")}
                }
            }

            div { class: "row-menu-sep" }

            if !caps.can_add_inside.is_hidden() {
                button {
                    class: "row-menu-item",
                    role: "menuitem",
                    disabled: !caps.can_add_inside.is_allowed(),
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        let _ = session.write().focus(node_id);
                        sync_draft(&session, &mut draft.clone());
                        status.clone().set("struct.split.pending".into());
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.add_inside")}
                }
            }
            if !caps.can_join_with_previous.is_hidden() {
                button {
                    class: "row-menu-item",
                    role: "menuitem",
                    disabled: !caps.can_join_with_previous.is_allowed(),
                    title: disabled_title(lang, &caps.can_join_with_previous),
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        let _ = session.write().focus(node_id);
                        if session.write().merge_focused_up().is_ok() {
                            sync_draft(&session, &mut draft.clone());
                        }
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.join_with_previous")}
                }
            }

            div { class: "row-menu-sep" }

            if !caps.can_delete.is_hidden() {
                button {
                    class: "row-menu-item row-menu-item--danger",
                    role: "menuitem",
                    disabled: !caps.can_delete.is_allowed(),
                    onclick: move |_| {
                        commit_draft_if_dirty(&mut session.clone(), &mut draft.clone());
                        let _ = session.write().focus(node_id);
                        sync_draft(&session, &mut draft.clone());
                        status.clone().set("struct.delete.pending".into());
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.delete")}
                }
            }
            if !caps.can_show_plain_text.is_hidden() {
                button {
                    class: "row-menu-item",
                    role: "menuitem",
                    onclick: move |_| {
                        session.write().show_raw();
                        menu_open_for.set(None);
                    },
                    {t(lang, "document_map.action.show_plain_text")}
                }
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn disabled_title(lang: Locale, cap: &MapCapability) -> String {
    if let MapCapability::Disabled(r) = cap {
        t(lang, r.catalog_key()).to_string()
    } else {
        String::new()
    }
}

fn find_node(root: &DocumentMapNode, id: u64) -> Option<&DocumentMapNode> {
    if root.id == id {
        return Some(root);
    }
    root.children.iter().find_map(|c| find_node(c, id))
}

/// Collect the `SwNodeId`s of all ancestors of `target` (root → parent,
/// not including `target` itself) into `out`. Returns `true` if found.
fn collect_ancestors(node: &DocumentMapNode, target: SwNodeId, out: &mut Vec<SwNodeId>) -> bool {
    if SwNodeId(node.id) == target {
        return true;
    }
    for child in &node.children {
        if collect_ancestors(child, target, out) {
            out.push(SwNodeId(node.id));
            return true;
        }
    }
    false
}

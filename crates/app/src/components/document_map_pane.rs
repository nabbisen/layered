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
use dioxus_swdir_tree::{ItemTree, ItemTreeEvent, ItemTreeView};
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
        // Read session once; build both derived representations.
        let root_node = session.read().document_map_nodes();
        let view = session.read().view_mode();

        item_tree.write().set_tree(to_item_node(&root_node));

        // After every tree update, ensure all ancestors of the currently
        // focused node are expanded. This covers both the normal navigation
        // case and the "node just added" case — when a brand-new node enters
        // set_tree it defaults to is_expanded=false, so we must force-expand
        // its parents to make it visible. Expanding a leaf is a no-op.
        if let ViewMode::Focus(focused_id) = view {
            let focused_sw = SwNodeId(focused_id.0);
            // Collect ancestor ids from the DocumentMapNode tree.
            let mut ancestors: Vec<SwNodeId> = Vec::new();
            collect_ancestors(&root_node, focused_sw, &mut ancestors);
            // Also include the focused node itself in case it has children
            // that were just added (the existing behaviour).
            ancestors.push(focused_sw);
            for id in ancestors {
                if item_tree.read().is_expanded(id) == Some(false) {
                    item_tree.write().on_toggled(id);
                }
            }
        }

        map_root_sig.set(Some(root_node));
    });

    let on_event = move |ev: ItemTreeEvent| match ev {
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
                div { class: "document-map-tree",
                    ItemTreeView { tree: item_tree, on_event }
                }
            }

            // Row menu for the node whose id is in `menu_open_for`.
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

            // ⋯ trigger buttons — one per non-root node, rendered flat.
            if let Some(root) = map_root.as_ref() {
                div { class: "document-map-menus",
                    {root.children.iter().map(|child| {
                        render_node_menu_trigger(child, menu_open_for)
                    })}
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

// ── Per-node ⋯ trigger (non-recursive free function returning Element) ────────

fn render_node_menu_trigger(
    node: &DocumentMapNode,
    mut menu_open_for: Signal<Option<u64>>,
) -> Element {
    let node_id = node.id;
    rsx! {
        span {
            key: "{node_id}-btn",
            class: "row-menu-trigger",
            button {
                class: "row-menu-btn",
                title: "Section actions",
                "aria-label": "Section actions",
                onclick: move |ev| {
                    ev.stop_propagation();
                    let cur = *menu_open_for.read();
                    menu_open_for.set(if cur == Some(node_id) { None } else { Some(node_id) });
                },
                "⋯"
            }
            // Render children triggers inline (avoids recursive `fn` call
            // inside rsx! which would create nested reactive scopes).
            {node.children.iter().map(|child| {
                render_node_menu_trigger(child, menu_open_for)
            })}
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

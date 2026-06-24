use super::*;

#[test]
fn plain_navigation_keys_map_without_modifiers() {
    assert_eq!(
        interpret_code(Code::Escape, false, false, false),
        Some(AppCommand::Escape)
    );
    assert_eq!(
        interpret_code(Code::Enter, false, false, false),
        Some(AppCommand::Enter)
    );
    assert_eq!(
        interpret_code(Code::ArrowUp, false, false, false),
        Some(AppCommand::SelectUp)
    );
    assert_eq!(
        interpret_code(Code::ArrowDown, false, false, false),
        Some(AppCommand::SelectDown)
    );
}

#[test]
fn ctrl_shortcuts_require_ctrl_and_reject_extra_modifiers() {
    assert_eq!(
        interpret_code(Code::KeyS, true, false, false),
        Some(AppCommand::Save)
    );
    // Without ctrl, a bare 'S' is a literal character, not a shortcut.
    assert_eq!(interpret_code(Code::KeyS, false, false, false), None);
    // Ctrl+Alt+S is not a defined binding.
    assert_eq!(interpret_code(Code::KeyS, true, false, true), None);
}

#[test]
fn shift_distinguishes_save_from_save_as() {
    assert_eq!(
        interpret_code(Code::KeyS, true, false, false),
        Some(AppCommand::Save)
    );
    assert_eq!(
        interpret_code(Code::KeyS, true, true, false),
        Some(AppCommand::SaveAs)
    );
}

#[test]
fn both_redo_bindings_resolve() {
    // Ctrl+Y and Ctrl+Shift+Z are both Redo.
    assert_eq!(
        interpret_code(Code::KeyY, true, false, false),
        Some(AppCommand::Redo)
    );
    assert_eq!(
        interpret_code(Code::KeyZ, true, true, false),
        Some(AppCommand::Redo)
    );
    // Ctrl+Z alone is Undo.
    assert_eq!(
        interpret_code(Code::KeyZ, true, false, false),
        Some(AppCommand::Undo)
    );
}

#[test]
fn shift_distinguishes_palette_from_preview() {
    assert_eq!(
        interpret_code(Code::KeyP, true, false, false),
        Some(AppCommand::OpenPalette)
    );
    assert_eq!(
        interpret_code(Code::KeyP, true, true, false),
        Some(AppCommand::TogglePreview)
    );
}

#[test]
fn alt_arrows_navigate_history() {
    assert_eq!(
        interpret_code(Code::ArrowLeft, false, false, true),
        Some(AppCommand::Back)
    );
    assert_eq!(
        interpret_code(Code::ArrowRight, false, false, true),
        Some(AppCommand::Forward)
    );
    // Plain arrows (no alt) are selection movement, handled elsewhere.
    assert_eq!(interpret_code(Code::ArrowLeft, false, false, false), None);
}

#[test]
fn unmapped_keys_return_none() {
    assert_eq!(interpret_code(Code::KeyQ, true, false, false), None);
    assert_eq!(interpret_code(Code::Tab, false, false, false), None);
}

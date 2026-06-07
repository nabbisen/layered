//! Desktop GUI components (RFC-010..013, RFC-017, RFC-019..026, RFC-027..029).

mod breadcrumb;
mod command_palette;
mod confirm_delete_dialog;
mod ext_modified_dialog;
mod focus_editor;
mod outline_pane;
mod overview_pane;
mod raw_source;
mod search_panel;
mod split_dialog;
mod status_bar;
mod toolbar;
mod unsaved_dialog;
mod welcome;

pub use breadcrumb::Breadcrumb;
pub use command_palette::{CommandId, CommandPalette};
pub use confirm_delete_dialog::{ConfirmDeleteChoice, ConfirmDeleteDialog};
pub use ext_modified_dialog::{ExtModifiedChoice, ExtModifiedDialog};
pub use focus_editor::FocusEditor;
pub use outline_pane::OutlinePane;
pub use overview_pane::OverviewPane;
pub use raw_source::RawSourceView;
pub use search_panel::SearchPanel;
pub use split_dialog::{SplitChoice, SplitDialog};
pub use status_bar::StatusBar;
pub use toolbar::Toolbar;
pub use unsaved_dialog::{UnsavedChoice, UnsavedDialog};
pub use welcome::WelcomeScreen;

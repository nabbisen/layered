use super::types::ParsedMarkdown;

#[tauri::command]
pub fn ready() -> String {
    // todo: open last opened ?
    crate::editor::dev::TEST_MARKDOWN.to_owned()
}

#[tauri::command]
pub fn open(filepath: &str) -> String {
    crate::editor::file::open(filepath)
}

#[tauri::command]
pub fn save(parsed_markdowns: Vec<ParsedMarkdown>, filepath: &str) -> () {
    crate::editor::file::save(parsed_markdowns, filepath)
}

#[tauri::command]
pub fn parse(markdown_text: &str) -> Vec<ParsedMarkdown> {
    crate::editor::content::parse(markdown_text)
}

#[tauri::command]
pub fn compose(parsed_markdowns: Vec<ParsedMarkdown>) -> String {
    crate::editor::content::compose(parsed_markdowns)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ParsedMarkdown {
    pub node_id: usize,
    pub ancestors: Vec<usize>,
    pub nesting_level: usize,
    pub heading_level: Option<usize>,
    pub heading_text: Option<String>,
    pub html: Option<String>,
}

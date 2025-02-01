use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ParsedMarkdown {
    pub node_id: usize,
    pub heading_level: usize,
    pub is_heading: bool,
    pub text: Option<String>,
    pub parent_node_id: Option<usize>,
    pub ancestors: Vec<usize>,
}

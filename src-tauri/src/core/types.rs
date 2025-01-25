use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContentType {
    pub text: String,
    // recursion
    pub children: Option<Vec<ContentType>>,
}

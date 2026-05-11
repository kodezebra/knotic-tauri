use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tutorial {
    pub id: String,
    pub title: String,
    pub content: String,
    pub metadata: TutorialMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TutorialMeta {
    pub author: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
}

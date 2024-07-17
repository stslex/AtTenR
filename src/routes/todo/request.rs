use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ToDoCreateRequest<'a> {
    #[serde(rename = "title")]
    pub title: &'a str,
    #[serde(rename = "description")]
    pub description: &'a str,
    #[serde(rename = "expired_at")]
    pub expired_at: i64,
}

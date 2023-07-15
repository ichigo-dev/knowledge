use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Term
{
    pub term_id: usize,
    pub path: String,
    pub term: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

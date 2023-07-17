use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct User
{
    pub user_id: usize,
    pub code: String,
    pub created_at: String,
}

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

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct UserResult
{
    pub user_result_id: usize,
    pub user_id: usize,
    pub created_at: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct UserAnswer
{
    pub user_answer_id: usize,
    pub user_result_id: usize,
    pub term_id: usize,
    pub is_correct: bool,
    pub created_at: String,
}

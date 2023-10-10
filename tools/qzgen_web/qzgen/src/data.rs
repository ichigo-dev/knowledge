use serde::{ Serialize, Deserialize, Deserializer };

#[derive(Debug, Default)]
pub struct Quiz
{
    pub quiz: String,
    pub term: Term,
    pub answer_path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User
{
    pub user_id: usize,
    pub code: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct Term
{
    pub term_id: usize,
    pub path: String,
    pub term: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct UserResult
{
    pub user_result_id: usize,
    pub user_id: usize,
    pub created_at: String,

    #[serde(default)]
    pub answers: Vec<UserAnswer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct UserAnswer
{
    pub user_answer_id: usize,
    pub user_result_id: usize,
    pub term_id: usize,

    #[serde(default)]
    pub term: Term,

    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub is_correct: bool,

    pub created_at: String,
}

fn deserialize_bool_from_int<'de, D>
(
    deserializer: D,
) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i32 = Deserialize::deserialize(deserializer)?;
    Ok(value != 0)
}

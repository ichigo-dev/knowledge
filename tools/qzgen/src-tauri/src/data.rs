/*

    Application Data.

*/

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Term
{
    term_id: i64,
    term: String,
}

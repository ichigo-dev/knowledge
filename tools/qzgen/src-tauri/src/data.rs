/*

    Application Data.

*/

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Term
{
    term_id: i64,
    term: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mask
{
    mask_id: i64,
    term_id: i64,
    mask: String,
}

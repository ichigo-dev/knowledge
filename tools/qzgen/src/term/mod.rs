mod dictionary;

pub(crate) use dictionary::Dictionary;

//------------------------------------------------------------------------------
/// Term.
//------------------------------------------------------------------------------
pub(crate) struct Term
{
    file_path: String,
    word: String,
    count: usize,
}

mod dictionary;

pub(crate) use dictionary::Dictionary;

//------------------------------------------------------------------------------
/// Term.
//------------------------------------------------------------------------------
pub(crate) struct Term
{
    word: String,
    description: String,
    source: String,
    children: Vec<Term>,
}

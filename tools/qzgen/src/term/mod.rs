mod dictionary;

pub(crate) use dictionary::Dictionary;
use crate::quiz::Quiz;

//------------------------------------------------------------------------------
/// Term.
//------------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Term
{
    term: String,
    content: String,
    source: String,
}

impl Term
{
    //--------------------------------------------------------------------------
    /// Creates a new term.
    //--------------------------------------------------------------------------
    pub(crate) fn new( term: &str, content: &str, source: &str ) -> Self
    {
        Self
        {
            term: term.to_string(),
            content: content.to_string(),
            source: source.to_string(),
        }
    }

    //--------------------------------------------------------------------------
    /// Creates a new quiz from the term.
    //--------------------------------------------------------------------------
    pub(crate) fn to_quiz( &self ) -> Quiz
    {
        Quiz::new(&self.term, &self.content, &self.source)
    }
}

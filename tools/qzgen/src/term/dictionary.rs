use crate::term::Term;

//------------------------------------------------------------------------------
/// Dictionary of terms.
//------------------------------------------------------------------------------
pub(crate) struct Dictionary
{
    terms: Vec<Term>,
}

impl Dictionary
{
    //--------------------------------------------------------------------------
    /// Creates a new dictionary.
    //--------------------------------------------------------------------------
    pub(crate) fn new() -> Self
    {
        Self { terms: Vec::new() }
    }

    //--------------------------------------------------------------------------
    /// Adds a term to the dictionary.
    //--------------------------------------------------------------------------
    pub(crate) fn add( &mut self, term: Term )
    {
        self.terms.push(term);
    }

    //--------------------------------------------------------------------------
    /// Merges two dictionaries.
    //--------------------------------------------------------------------------
    pub(crate) fn merge( &mut self, other: Dictionary )
    {
        self.terms.extend(other.terms);
    }
}

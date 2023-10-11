//------------------------------------------------------------------------------
//! Dictionary of terms.
//------------------------------------------------------------------------------

use crate::term::Term;

use rand::Rng;

//------------------------------------------------------------------------------
/// Dictionary of terms.
//------------------------------------------------------------------------------
#[derive(Debug)]
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

    //--------------------------------------------------------------------------
    /// Gets the number of terms in the dictionary.
    //--------------------------------------------------------------------------
    pub(crate) fn len( &self ) -> usize
    {
        self.terms.len()
    }

    //--------------------------------------------------------------------------
    /// Gets a term from the dictionary.
    //--------------------------------------------------------------------------
    pub(crate) fn get( &self, index: usize ) -> Option<&Term>
    {
        self.terms.get(index)
    }

    //--------------------------------------------------------------------------
    /// Gets a random term from the dictionary.
    //--------------------------------------------------------------------------
    pub(crate) fn get_random( &self ) -> Option<&Term>
    {
        if self.len() == 0
        {
            return None;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.len());
        self.terms.get(index)
    }
}

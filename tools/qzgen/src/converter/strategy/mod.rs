//------------------------------------------------------------------------------
//! A strategy for somehow converting a data source into a dictionary of terms.
//! A concrete implementation of this strategy should have a convert method
//! that takes a DataSource and returns a Dictionary.
//------------------------------------------------------------------------------

mod markdown;

pub(crate) use markdown::MarkdownConvertStrategy;

use crate::data_source::DataSource;
use crate::term::Dictionary;

//------------------------------------------------------------------------------
/// Interface for strategies when converting data sources to dictionary.
//------------------------------------------------------------------------------
pub(crate) enum ConvertStrategy
{
    Markdown(MarkdownConvertStrategy),
}

impl ConvertStrategy
{
    //--------------------------------------------------------------------------
    /// Convert data source to word list.
    //--------------------------------------------------------------------------
    pub(super) fn convert( &self, data_source: DataSource ) -> Dictionary
    {
        match self
        {
            ConvertStrategy::Markdown(strategy) =>
            {
                strategy.convert(data_source)
            },
        }
    }
}

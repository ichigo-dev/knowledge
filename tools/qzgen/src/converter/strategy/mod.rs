//------------------------------------------------------------------------------
//! A strategy for somehow converting a data source into a dictionary of terms.
//------------------------------------------------------------------------------

mod markdown;

pub(crate) use markdown::MarkdownConvertStrategy;

use crate::data_source::DataSource;
use crate::term::Dictionary;

//------------------------------------------------------------------------------
/// Interface for strategies when converting data sources to dictionary.
//------------------------------------------------------------------------------
pub(crate) trait ConvertStrategy
{
    //--------------------------------------------------------------------------
    /// Convert data source to word list.
    //--------------------------------------------------------------------------
    fn convert( &self, data_source: DataSource ) -> Dictionary;
}

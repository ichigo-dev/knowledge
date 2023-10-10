mod strategy;

use strategy::{ ConvertStrategy, MarkDownConvertStrategy };
use crate::data_source::DataSource;
use crate::term::Dictionary;

//------------------------------------------------------------------------------
/// Converter to convert data source to dictionary.
//------------------------------------------------------------------------------
pub(crate) struct Converter
{
    strategy: Box<dyn ConvertStrategy>,
}

impl Converter
{
    //--------------------------------------------------------------------------
    /// Creates a new converter.
    //--------------------------------------------------------------------------
    pub(crate) fn new( strategy: Box<dyn ConvertStrategy> ) -> Self
    {
        Self { strategy }
    }

    //--------------------------------------------------------------------------
    /// Converts data source to dictionary.
    //--------------------------------------------------------------------------
    pub(crate) fn convert( &self, data_source: DataSource ) -> Dictionary
    {
        self.strategy.convert(data_source)
    }
}

impl Default for Converter
{
    fn default() -> Self
    {
        Self::new(Box::new(MarkDownConvertStrategy))
    }
}

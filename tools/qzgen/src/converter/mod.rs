//------------------------------------------------------------------------------
//! A Converter that transforms a data source into a dictionary of terms,
//! depending on a strategy.
//!
//! # Example
//!
//! ```
//! use std::path::Path;
//! use crate::data_source::{ DataSource, FileDataSource };
//! use crate::converter::Converter;
//!
//! let path = Path::new("note.md");
//! let data_source = DataSource::File(FileDataSource::new(path));
//!
//! let converter = Converter::default();
//! converter.convert(data_source);
//! ```
//------------------------------------------------------------------------------

mod strategy;

pub(crate) use strategy::{ ConvertStrategy, MarkdownConvertStrategy };
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
        Self::new(Box::new(MarkdownConvertStrategy))
    }
}

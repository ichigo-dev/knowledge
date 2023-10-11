//------------------------------------------------------------------------------
//! Data sources stored in files, databases, etc.
//!
//! # Example
//!
//! ```
//! use std::path::Path;
//! use crate::data_source::{ DataSource, FileDataSource };
//!
//! let path = Path::new("note.md");
//! let data_source = DataSource::File(FileDataSource::new(path));
//! ```
//------------------------------------------------------------------------------

mod file;

pub(crate) use file::FileDataSource;

use std::path::Path;

//------------------------------------------------------------------------------
/// Data source to convert to dictionary.
//------------------------------------------------------------------------------
pub(crate) enum DataSource
{
    File(FileDataSource),
}

impl DataSource
{
    //--------------------------------------------------------------------------
    /// Gets the content of the data source.
    //--------------------------------------------------------------------------
    pub(crate) fn get_content( &self ) -> &str
    {
        match self
        {
            Self::File(file_data_source) => file_data_source.get_content(),
        }
    }
}

impl Default for DataSource
{
    fn default() -> Self
    {
        Self::File(FileDataSource::default())
    }
}

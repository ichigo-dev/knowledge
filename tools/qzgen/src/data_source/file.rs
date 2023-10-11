//------------------------------------------------------------------------------
//! File data source.
//------------------------------------------------------------------------------

use std::fs::File;
use std::path::Path;
use std::io::Read;

//------------------------------------------------------------------------------
/// Data source from file.
//------------------------------------------------------------------------------
pub(crate) struct FileDataSource
{
    file: Option<File>,
    file_path: String,
    content: String,
}

impl FileDataSource
{
    //--------------------------------------------------------------------------
    /// Creates a new data source from a file.
    //--------------------------------------------------------------------------
    pub(crate) fn new( path: &Path ) -> Self
    {
        let mut file_data_source = Self::default();
        file_data_source.read(path);
        file_data_source
    }

    //--------------------------------------------------------------------------
    /// Reads the data source from a file.
    //--------------------------------------------------------------------------
    pub(super) fn read( &mut self, path: &Path )
    {
        let mut file = File::open(path).expect("Failed to open file.");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file.");

        self.file = Some(file);
        self.file_path = path.to_str().unwrap().to_string();
        self.content = content;
    }

    //--------------------------------------------------------------------------
    /// Gets the content of the data source.
    //--------------------------------------------------------------------------
    pub(super) fn get_content( &self ) -> String
    {
        self.content.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the source of the data source.
    //--------------------------------------------------------------------------
    pub(super) fn get_source( &self ) -> String
    {
        self.file_path.clone()
    }
}

impl Default for FileDataSource
{
    fn default() -> Self
    {
        Self
        {
            file: None,
            file_path: String::new(),
            content: String::new(),
        }
    }
}

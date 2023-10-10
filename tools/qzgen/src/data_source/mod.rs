use std::fs::File;
use std::path::Path;
use std::io::Read;

//------------------------------------------------------------------------------
/// Structure representing the data source for retrieving a list of terms.
//------------------------------------------------------------------------------
pub(crate) struct DataSource
{
    file: File,
    file_path: String,
    content: String,
}

impl DataSource
{
    //--------------------------------------------------------------------------
    /// Creates a new data source.
    //--------------------------------------------------------------------------
    pub(crate) fn new( path: &Path ) -> Self
    {
        let mut file = File::open(path).expect("Failed to open file.");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file.");
        Self
        {
            file,
            file_path: path.to_str().unwrap().to_string(),
            content,
        }
    }

    //--------------------------------------------------------------------------
    /// Gets the content of the data source.
    //--------------------------------------------------------------------------
    pub(crate) fn get_content( &self ) -> &str
    {
        &self.content
    }
}

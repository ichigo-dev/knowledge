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

//------------------------------------------------------------------------------
/// MarkDown strategy for converting data sources to dictionary.
//------------------------------------------------------------------------------
pub(crate) struct MarkDownConvertStrategy;

impl MarkDownConvertStrategy
{
    //--------------------------------------------------------------------------
    /// Gets the depth of the section.
    //--------------------------------------------------------------------------
    fn get_section_depth( &self, line: &str ) -> usize
    {
        let mut depth = 0;
        for c in line.chars()
        {
            if c == '#'
            {
                depth += 1;
            }
            else
            {
                break;
            }
        }
        depth
    }

    //--------------------------------------------------------------------------
    /// Gets the name of the section.
    //--------------------------------------------------------------------------
    fn get_section_name( &self, line: &str ) -> String
    {
        line.trim_start_matches('#').trim().to_string()
    }

    //--------------------------------------------------------------------------
    /// Splits the data source into sections.
    //--------------------------------------------------------------------------
    fn split_sections( &self, data_source: DataSource ) -> Vec<String>
    {
        let mut sections = Vec::new();
        let mut section = String::new();
        for line in data_source.get_content().lines()
        {
            if line.starts_with("#")
            {
                if !section.is_empty()
                {
                    sections.push(section);
                    section = String::new();
                }
            }
            section.push_str(line);
            section.push('\n');
        }
        sections.push(section);
        sections
    }

    //--------------------------------------------------------------------------
    /// Parses the sections of the data source.
    //--------------------------------------------------------------------------
    fn parse_sections( &self, data_source: DataSource ) -> Vec<MarkDownSection>
    {
        let mut sections = Vec::new();
        for section in self.split_sections(data_source)
        {
            let mut lines = section.lines();
            let heading_line = lines.next().unwrap();
            let depth = self.get_section_depth(heading_line);
            let name = self.get_section_name(heading_line);
            let mut content = lines.collect::<Vec<&str>>().join("\n");
            sections.push(MarkDownSection::new(depth, &name, &content.trim()));
        }
        sections
    }
}

impl ConvertStrategy for MarkDownConvertStrategy
{
    fn convert( &self, data_source: DataSource ) -> Dictionary
    {
        let mut dictionary = Dictionary::new();
        println!("{:#?}", self.parse_sections(data_source));
        dictionary
    }
}

//------------------------------------------------------------------------------
/// Structure representing a section of a MarkDown file.
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub(crate) struct MarkDownSection
{
    depth: usize,
    name: String,
    content: String,
}

impl MarkDownSection
{
    //--------------------------------------------------------------------------
    /// Creates a new MarkDown section.
    //--------------------------------------------------------------------------
    pub(crate) fn new( depth: usize, name: &str, content: &str ) -> Self
    {
        Self
        {
            depth: depth,
            name: name.to_string(),
            content: content.to_string(),
        }
    }
}

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
/// Markdown strategy for converting data sources to dictionary.
//------------------------------------------------------------------------------
pub(crate) struct MarkdownConvertStrategy;

impl MarkdownConvertStrategy
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
    /// Parses a section of the data source.
    //--------------------------------------------------------------------------
    fn parse_section( &self, section_str: &str ) -> MarkdownSection
    {
        let mut lines = section_str.lines();
        let heading_line = lines.next().unwrap();
        let depth = self.get_section_depth(heading_line);
        let name = self.get_section_name(heading_line);
        let mut content = lines.collect::<Vec<&str>>().join("\n");
        MarkdownSection::new(depth, &name, &content.trim())
    }

    //--------------------------------------------------------------------------
    /// Parses the sections of the data source.
    //--------------------------------------------------------------------------
    fn parse_sections( &self, data_source: DataSource ) -> Option<MarkdownSection>
    {
        unimplemented!();
    }
}

impl ConvertStrategy for MarkdownConvertStrategy
{
    fn convert( &self, data_source: DataSource ) -> Dictionary
    {
        let mut dictionary = Dictionary::new();
        println!("{:#?}", self.parse_sections(data_source));
        dictionary
    }
}

//------------------------------------------------------------------------------
/// Structure representing a section of a Markdown file.
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub(crate) struct MarkdownSection
{
    children: Vec<MarkdownSection>,
    depth: usize,
    name: String,
    content: String,
}

impl MarkdownSection
{
    //--------------------------------------------------------------------------
    /// Creates a new Markdown section.
    //--------------------------------------------------------------------------
    pub(crate) fn new( depth: usize, name: &str, content: &str ) -> Self
    {
        Self
        {
            children: Vec::new(),
            depth: depth,
            name: name.to_string(),
            content: content.to_string(),
        }
    }

    //--------------------------------------------------------------------------
    /// Adds a child section.
    //--------------------------------------------------------------------------
    pub(crate) fn add_child( &mut self, child: MarkdownSection )
    {
        self.children.push(child);
    }
}

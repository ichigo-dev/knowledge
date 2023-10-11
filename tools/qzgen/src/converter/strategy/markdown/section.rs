//------------------------------------------------------------------------------
//! Structure representing a Markdown section.
//!
//! One section is structured as follows.
//!
//! ```markdown
//! # Section Name
//!
//! Section content.
//! ```
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
/// Structure representing a section of a Markdown file.
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub(super) struct MarkdownSection
{
    raw_content: String,
    name: String,
    content: String,
    depth: usize,
}

impl MarkdownSection
{
    //--------------------------------------------------------------------------
    /// Creates a new Markdown section.
    //--------------------------------------------------------------------------
    pub(super) fn new( raw_content: &str ) -> Self
    {
        let mut section = Self::default();
        section.raw_content = raw_content.to_string();
        section.parse();
        section
    }

    //--------------------------------------------------------------------------
    /// Parses raw content into a Markdown section.
    //--------------------------------------------------------------------------
    fn parse( &mut self )
    {
        let mut lines = self.raw_content.lines();
        let heading_line = lines.next().unwrap();
        let mut content = lines.collect::<Vec<&str>>().join("\n");

        self.depth = self.parse_section_depth(heading_line);
        self.name = heading_line.trim_start_matches('#').trim().to_string();
        self.content = content.trim().to_string();
    }

    //--------------------------------------------------------------------------
    /// Gets the depth of the section.
    //--------------------------------------------------------------------------
    fn parse_section_depth( &self, line: &str ) -> usize
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
    /// Gets the depth of the section.
    //--------------------------------------------------------------------------
    pub(super) fn depth( &self ) -> usize
    {
        self.depth
    }
}

impl Default for MarkdownSection
{
    fn default() -> Self
    {
        Self
        {
            raw_content: String::new(),
            name: String::new(),
            content: String::new(),
            depth: 0,
        }
    }
}

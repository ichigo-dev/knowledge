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

use regex::Regex;

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
        let content = lines.collect::<Vec<&str>>().join("\n");

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
    /// Gets the content in the section.
    //--------------------------------------------------------------------------
    pub(super) fn get_content( &self ) -> String
    {
        self.content.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the terms in the section.
    //--------------------------------------------------------------------------
    pub(super) fn get_terms( &self ) -> Vec<String>
    {
        let re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
        let mut terms = Vec::new();
        for capture in re.captures_iter(&self.content)
        {
            if let Some(term) = capture.get(1)
            {
                if terms.contains(&term.as_str().to_string())
                {
                    continue;
                }

                terms.push(term.as_str().to_string());
            }
        }
        terms
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

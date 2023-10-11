//------------------------------------------------------------------------------
//! Strategy for converting Markdown formatted data sources into a dictionary
//! of terms.
//! 
//! Register highlighted character strings written in Markdown sentences as
//! terms in the dictionary. The `source` is a file path with a heading added.
//------------------------------------------------------------------------------

mod section;

use section::MarkdownSection;
use crate::data_source::DataSource;
use crate::term::Dictionary;
use crate::converter::ConvertStrategy;

//------------------------------------------------------------------------------
/// Markdown strategy for converting data sources to dictionary.
//------------------------------------------------------------------------------
pub(crate) struct MarkdownConvertStrategy;

impl ConvertStrategy for MarkdownConvertStrategy
{
    fn convert( &self, data_source: DataSource ) -> Dictionary
    {
        let mut dictionary = Dictionary::new();
        println!("{:#?}", self.parse_sections(data_source));
        dictionary
    }
}

impl MarkdownConvertStrategy
{
    //--------------------------------------------------------------------------
    /// Parses the sections of the data source.
    //--------------------------------------------------------------------------
    fn parse_sections
    (
        &self,
        data_source: DataSource,
    ) -> Option<MarkdownSection>
    {
        let sections = self.split_sections(data_source)
            .into_iter()
            .map(|s| MarkdownSection::new(&s))
            .collect::<Vec<MarkdownSection>>();

        for section in sections
        {
        }

        unimplemented!();
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
}

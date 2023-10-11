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

impl MarkdownConvertStrategy
{
    //--------------------------------------------------------------------------
    /// Convert data source to word list.
    //--------------------------------------------------------------------------
    pub(super) fn convert( &self, data_source: DataSource ) -> Dictionary
    {
        let mut dictionary = Dictionary::new();
        let sections = self.parse_sections(data_source);
        //println!("{:#?}", sections);
        dictionary
    }

    //--------------------------------------------------------------------------
    /// Parses the sections of the data source.
    //--------------------------------------------------------------------------
    fn parse_sections
    (
        &self,
        data_source: DataSource,
    ) -> Vec<MarkdownSection>
    {
        let mut sections = self.split_sections(data_source);
        println!("{:#?}", sections);

        unimplemented!();
    }

    //--------------------------------------------------------------------------
    /// Splits the data source into sections.
    //--------------------------------------------------------------------------
    fn split_sections( &self, data_source: DataSource ) -> Vec<MarkdownSection>
    {
        let mut sections = Vec::new();
        let mut raw_section = String::new();
        for line in data_source.get_content().lines()
        {
            if line.starts_with("#")
            {
                if !raw_section.is_empty()
                {
                    sections.push(MarkdownSection::new(&raw_section));
                    raw_section = String::new();
                }
            }
            raw_section.push_str(line);
            raw_section.push('\n');
        }
        let section = MarkdownSection::new(&raw_section);
        sections.push(section);
        sections
    }
}

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
use crate::term::{ Term, Dictionary };

//------------------------------------------------------------------------------
/// Markdown strategy for converting data sources to dictionary.
//------------------------------------------------------------------------------
pub(crate) struct MarkdownConvertStrategy;

impl MarkdownConvertStrategy
{
    //--------------------------------------------------------------------------
    /// Convert data source to term list.
    //--------------------------------------------------------------------------
    pub(super) fn convert( &self, data_source: DataSource ) -> Dictionary
    {
        let mut dictionary = Dictionary::new();
        let content = data_source.get_content();
        let sections = self.split_sections(&content);
        for section in sections
        {
            let terms = section.get_terms();
            for term in terms
            {
                let term = Term::new
                (
                    &term,
                    &section.get_content(),
                    &data_source.get_source(),
                );
                dictionary.add(term);
            }
        }
        dictionary
    }

    //--------------------------------------------------------------------------
    /// Splits the data source into sections.
    //--------------------------------------------------------------------------
    fn split_sections( &self, content: &str ) -> Vec<MarkdownSection>
    {
        let mut sections = Vec::new();
        let mut raw_section = String::new();
        for line in content.lines()
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

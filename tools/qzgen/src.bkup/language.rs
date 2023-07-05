use std::default::Default;
use std::convert::From;

#[derive(PartialEq, Clone)]
pub enum Language
{
    Japanese,
    English,
}

impl Language
{
    pub fn get_all_languages() -> Vec<Language>
    {
        vec!
        [
            Language::Japanese,
            Language::English,
        ]
    }

    pub fn code( &self ) -> String
    {
        match self
        {
            Language::Japanese => "ja".to_string(),
            Language::English => "en".to_string(),
        }
    }

    pub fn label( &self ) -> String
    {
        match self
        {
            Language::Japanese => "Japanese".to_string(),
            Language::English => "English".to_string(),
        }
    }
}

impl Default for Language
{
    fn default() -> Self
    {
        Language::Japanese
    }
}

impl From<&str> for Language
{
    fn from( code: &str ) -> Self
    {
        match code
        {
            "ja" => Language::Japanese,
            "en" => Language::English,
            _ => Language::default(),
        }
    }
}

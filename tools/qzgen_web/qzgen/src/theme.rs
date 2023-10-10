/*

    Application theme.

*/

use std::default::Default;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Theme
{
    Light,
    Dark,
}

impl Theme
{
    //--------------------------------------------------------------------------
    //  Gets the theme class name.
    //--------------------------------------------------------------------------
    pub fn class_name( &self ) -> &str
    {
        match self
        {
            Theme::Light => "theme_light",
            Theme::Dark => "theme_dark",
        }
    }
}

impl Default for Theme
{
    //--------------------------------------------------------------------------
    //  Gets the default theme.
    //--------------------------------------------------------------------------
    fn default() -> Self
    {
        Theme::Light
    }
}

impl FromStr for Theme
{
    type Err = ();

    //--------------------------------------------------------------------------
    //  Parses a theme from a string.
    //--------------------------------------------------------------------------
    fn from_str( s: &str ) -> Result<Self, Self::Err>
    {
        match s
        {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Theme
{
    //--------------------------------------------------------------------------
    //  Formats a theme to a string.
    //--------------------------------------------------------------------------
    fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
    {
        match self
        {
            Theme::Light => write!( f, "light" ),
            Theme::Dark => write!( f, "dark" ),
        }
    }
}

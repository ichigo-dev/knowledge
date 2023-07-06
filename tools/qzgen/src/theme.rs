/*

    Application theme.

*/

use std::default::Default;

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

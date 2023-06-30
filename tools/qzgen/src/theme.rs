use std::default::Default;

pub enum Theme
{
    Light,
    Dark,
}

impl Theme
{
    pub fn get_class_name( &self ) -> &str
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
    fn default() -> Self
    {
        Theme::Light
    }
}

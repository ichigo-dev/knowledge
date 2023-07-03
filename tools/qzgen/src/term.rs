#[derive(Debug)]
pub struct Term
{
    name: String,
    link: String,
}

impl Term
{
    pub fn new( name: &str, link: &str ) -> Self
    {
        Term
        {
            name: name.to_string(),
            link: link.to_string(),
        }
    }
}

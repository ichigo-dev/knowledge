#[derive(Debug)]
pub struct Term
{
    term: String,
    path: String,
}

impl Term
{
    pub fn new( term: String, path: String ) -> Self
    {
        Term
        {
            term,
            path,
        }
    }

    pub fn term( &self ) -> &str
    {
        self.term.as_str()
    }

    pub fn path( &self ) -> &str
    {
        self.path.as_str()
    }
}

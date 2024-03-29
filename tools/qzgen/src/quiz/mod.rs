mod score;
mod judge;

pub(crate) use score::Score;
pub(crate) use judge::Judge;


//------------------------------------------------------------------------------
/// Quiz.
//------------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Quiz
{
    answer: String,
    content: String,
    source: String,
}

impl Quiz
{
    //--------------------------------------------------------------------------
    /// Creates a new quiz.
    //--------------------------------------------------------------------------
    pub(crate) fn new( answer: &str, content: &str, source: &str ) -> Self
    {
        let mut mask = String::new();
        mask.push_str("<span class=\"mask\">");
        mask.push_str(answer);
        mask.push_str("</span>");

        let content = content.replace(answer, &mask);
        Self
        {
            answer: answer.to_string(),
            content: content.to_string(),
            source: source.to_string(),
        }
    }

    //--------------------------------------------------------------------------
    /// Gets the answer of the quiz.
    //--------------------------------------------------------------------------
    pub(crate) fn get_answer( &self ) -> String
    {
        self.answer.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the content of the quiz.
    //--------------------------------------------------------------------------
    pub(crate) fn get_content( &self ) -> String
    {
        self.content.clone()
    }

    //--------------------------------------------------------------------------
    /// Challenges answer the quiz.
    //--------------------------------------------------------------------------
    pub(crate) fn challenge_answer( &self, answer: &str ) -> bool
    {
        self.answer == answer
    }

    //--------------------------------------------------------------------------
    /// Gets the source of the quiz.
    //--------------------------------------------------------------------------
    pub(crate) fn get_source( &self ) -> String
    {
        self.source.clone()
    }
}

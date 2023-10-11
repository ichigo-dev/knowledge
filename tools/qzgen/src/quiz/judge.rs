use crate::quiz::Quiz;

//------------------------------------------------------------------------------
// Judge.
//------------------------------------------------------------------------------
pub(crate) struct Judge
{
    max_try_count: usize,
    try_count: usize,
    result: bool,
}

impl Judge
{
    //--------------------------------------------------------------------------
    /// Creates a new judge.
    //--------------------------------------------------------------------------
    pub(crate) fn new( max_try_count: usize ) -> Self
    {
        Self
        {
            max_try_count,
            try_count: 0,
            result: false,
        }
    }

    //--------------------------------------------------------------------------
    /// Challenges answer the quiz.
    //--------------------------------------------------------------------------
    pub(crate) fn challenge_answer( &mut self, quiz: &Quiz ) -> bool
    {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        self.try_count += 1;
        self.result = quiz.challenge_answer(&input);
        self.result
    }

    //--------------------------------------------------------------------------
    /// Checks whether the quiz is continue.
    //--------------------------------------------------------------------------
    pub(crate) fn is_continue( &self ) -> bool
    {
        self.try_count < self.max_try_count
    }

    //--------------------------------------------------------------------------
    /// Gets the result.
    //--------------------------------------------------------------------------
    pub(crate) fn get_result( &self ) -> bool
    {
        self.result
    }
}

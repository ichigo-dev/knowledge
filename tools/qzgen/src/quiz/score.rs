use crate::quiz::Judge;

//------------------------------------------------------------------------------
// Score.
//------------------------------------------------------------------------------
pub(crate) struct Score
{
    correct: usize,
    incorrect: usize,
    max_try_count: usize,
}

impl Score
{
    //--------------------------------------------------------------------------
    /// Creates a new score.
    //--------------------------------------------------------------------------
    pub(crate) fn new( max_try_count: usize ) -> Self
    {
        Self
        {
            correct: 0,
            incorrect: 0,
            max_try_count,
        }
    }

    //--------------------------------------------------------------------------
    /// Gets the quiz judgement.
    //--------------------------------------------------------------------------
    pub(crate) fn get_judge( &self ) -> Judge
    {
        Judge::new(self.max_try_count)
    }

    //--------------------------------------------------------------------------
    /// Applies the quiz judgement.
    //--------------------------------------------------------------------------
    pub(crate) fn apply_judge( &mut self, judge: &Judge )
    {
        if judge.get_result()
        {
            self.correct += 1;
        }
        else
        {
            self.incorrect += 1;
        }
    }
}

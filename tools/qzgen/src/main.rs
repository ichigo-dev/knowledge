//------------------------------------------------------------------------------
//! Read data source and generate quiz.
//------------------------------------------------------------------------------

mod data_source;
mod converter;
mod term;
mod quiz;

use data_source::{ DataSource, FileDataSource };
use converter::Converter;
use term::Dictionary;
use quiz::Score;

use glob::glob;

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
fn main()
{
    // constants.
    let path = "../../note/**/*.md";
    let max_try_count = 3;

    let glob = glob(path)
        .expect("Failed to read glob pattern");
    let converter = Converter::default();
    let mut dictionary = Dictionary::new();

    for entry in glob
    {
        match entry
        {
            Ok(path) =>
            {
                let data_source = DataSource::File(FileDataSource::new(&path));
                let sub_dictinary = converter.convert(data_source);
                dictionary.merge(sub_dictinary);
            },
            Err(e) => println!("{:?}", e),
        }
    }

    let mut score = Score::new(max_try_count);
    for i in 0..10
    {
        let term = dictionary.get_random();
        let quiz = term.unwrap().to_quiz();
        let mut judge = score.get_judge();

        println!("{}", quiz.get_content());
        while judge.is_continue()
        {
            println!("Your answer: ");
            if judge.challenge_answer(&quiz)
            {
                break;
            }
        }
        score.apply_judge(&judge);
    }
}

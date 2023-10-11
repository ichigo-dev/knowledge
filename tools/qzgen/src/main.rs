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

use std::fs::File;
use std::io::Write;

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

    let step = 100;
    let chunks = (0..dictionary.len()).step_by(step);
    for (chunk_id, chunk) in chunks.enumerate()
    {
        let file_name = format!("quiz_{}.txt", chunk_id + 1);
        let mut file = File::create(&file_name).unwrap();
        for index in chunk..chunk + step
        {
            if index >= dictionary.len()
            {
                break;
            }

            let term = dictionary.get(index).unwrap();
            let quiz = term.to_quiz();
            let _ = file.write(b"====================\n\n").unwrap();
            let _ = file.write(quiz.get_content().as_bytes()).unwrap();
            let _ = file.write(b"\n\nAnswer: ");
            let _ = file.write(quiz.get_answer().as_bytes()).unwrap();
            let _ = file.write(b"\n\nSource: ");
            let _ = file.write(quiz.get_source().as_bytes()).unwrap();
            let _ = file.write(b"\n\n");
        }
    }

    let mut score = Score::new(max_try_count);
    for _ in 0..10
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

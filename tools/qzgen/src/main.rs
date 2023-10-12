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

    for index in 0..dictionary.len()
    {
        let term = dictionary.get(index).unwrap();
        let quiz = term.to_quiz();

        let html = format!(r#"
<!DOCTYPE html>
<html>
 <head>
  <meta charset="utf-8">
  <style>
   body
   {{
       display: flex;
       justify-content: center;
       min-height: 100vh;
       background-color: #444;
       color: #fff;
   }}

   .quiz_container
   {{
       line-height: 2;
   }}

   .mask
   {{
       background-color: #fff;
       color: #fff;
       padding: 2px 8px;
       margin: 0 4px;
   }}

   .mask.active
   {{
       color: #000;
   }}

   .btn_answer_container
   {{
       text-align: center;
   }}
  </style>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/kognise/water.css@latest/dist/dark.min.css">
 </head>
 <body>
  <div class="quiz_container">
   <p>{body}</p>
   <div class="btn_answer_container">
    <button id="btn_answer" class="btn_answer">Show answer</button>
   </div>
  </div>

  <script>
   const btn_answer = document.getElementById("btn_answer");
   btn_answer.addEventListener("click", function()
   {{
       const mask = document.querySelectorAll(".mask");
       mask.forEach(function(item)
       {{
           item.classList.toggle("active");
       }});
   }});
  </script>
 </body>
</html>
            "#,
            body = quiz.get_content(),
        );

        let file_name = format!("../../quiz/quiz_{}.html", index + 1);
        let mut file = File::create(&file_name).unwrap();
        let _ = file.write(html.as_bytes());
    }

    /*
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
    */
}

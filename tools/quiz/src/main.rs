use std::fs::File;
use std::io::{ self, Write, BufRead, BufReader };

use easy_reader::EasyReader;
use markdown_to_text;
use colored::Colorize;

//------------------------------------------------------------------------------
//  エラーをスキップしてcontinue
//------------------------------------------------------------------------------
macro_rules! skip_fail
{
    ( $res:expr ) =>
    {
        match $res
        {
            Ok(val) => val,
            Err(e) =>
            {
                println!("[Warning] {}", e);
                continue;
            }
        }
    };
}

//------------------------------------------------------------------------------
//  Noneをスキップしてcontinue
//------------------------------------------------------------------------------
macro_rules! skip_none
{
    ( $res:expr ) =>
    {
        match $res
        {
            Some(val) => val,
            None => { continue; },
        }
    };
}

//------------------------------------------------------------------------------
//  main
//------------------------------------------------------------------------------
fn main()
{
    let doc_root = String::from("note/ja/");
    let filename = "checksheet.md";

    //  ファイルを読み込む
    let file = File::open(doc_root.clone() + filename)
        .expect("file not found");
    let mut reader = EasyReader::new(file)
        .expect("something went wrong while reading the file");
    let _ = reader.build_index();

    let mut quiz_cnt = 0;
    let mut correct_cnt = 0;
    loop
    {
        //  ランダムに1行取得して、空行かヘッダ行ならスキップ
        let line = skip_fail!(reader.random_line());
        let line = skip_none!(line).trim().to_string();
        if line.len() <= 0 || line.starts_with("#") { continue; }

        //  問題を出題
        let items = line
            .rsplit(&['[', ']'])
            .collect::<Vec<_>>();
        if items.len() < 2 { continue; }

        let question_content = items[1];
        let answer_link = items[0];
        quiz_cnt += 1;
        println!
        (
            "\n{}{}{}{}{}\n",
            " Q-".on_bright_cyan().black(),
            quiz_cnt.to_string().on_bright_cyan().black(),
            ". ".on_bright_cyan().black(),
            &question_content.on_bright_cyan().black(),
            " ".on_bright_cyan()
        );

        //  回答を確認
        let mut hint_depth = 0;
        let mut hint_cnt = 0;
        'answer: loop
        {
            print!("Could you explain this term? [y/n/q(quit)] >> ");
            skip_fail!(std::io::stdout().flush());
            let mut result = String::new();
            skip_fail!(io::stdin().read_line(&mut result));

            match result.trim()
            {
                "y" =>
                {
                    correct_cnt += 1;
                    println!("\t{}\n", "Good".bold().green());
                    skip_fail!(show_answer(&doc_root, answer_link));
                    break;
                },
                "n" =>
                {
                    //  わからなければヒントを出す
                    hint_cnt += 1;
                    'hint: loop
                    {
                        //  前の行を探索してヘッダ行を探す
                        let prev_line = skip_fail!(reader.prev_line());
                        let prev_line = skip_none!(prev_line)
                            .trim()
                            .to_string();
                        if prev_line.starts_with("#") == false { continue; }

                        let header_depth = prev_line.rmatch_indices("#")
                            .collect::<Vec<_>>()
                            .len();
                        if hint_depth <= 0
                        {
                            hint_depth = header_depth + 1;
                        }

                        if header_depth < 2
                        {
                            //  最後のヒントに達していたら回答を表示
                            skip_fail!(show_answer(&doc_root, answer_link));
                            break 'answer;
                        }
                        else if header_depth < hint_depth
                        {
                            //  次のヒントを表示
                            hint_depth = header_depth;
                            println!
                            (
                                "\t{}{}{}{}\n",
                                "Hint-".yellow(),
                                hint_cnt.to_string().yellow(),
                                ". ".yellow(),
                                prev_line.trim_start_matches("#").trim().yellow()
                            );
                            break 'hint;
                        }
                    }
                },
                "q" =>
                {
                    println!
                    (
                        "\n{}{}{}{}",
                        "Result: ".green().bold(),
                        correct_cnt.to_string().green().bold(),
                        "/".green().bold(),
                        (quiz_cnt - 1).to_string().green().bold()
                    );

                    print!("Press Enter to finish");
                    skip_fail!(std::io::stdout().flush());
                    let mut fin = String::new();
                    let _ = io::stdin().read_line(&mut fin);

                    return;
                },
                _ => {},
            }
        }
    }
}

//------------------------------------------------------------------------------
//  回答を表示
//------------------------------------------------------------------------------
fn show_answer
(
    doc_root: &str,
    answer_link: &str,
) -> Result<(), std::io::Error>
{
    let answer = answer_link.replace("(./", "").replace(")", "");
    let answer_paths = answer.split("#").collect::<Vec<_>>();
    let answer_path = doc_root.to_string() + answer_paths[0];
    let answer_header = answer_paths[1];
    println!
    (
        "\t{}{}{}{}\n",
        "Answer. ".cyan(),
        &answer_path.cyan(),
        " >> ".cyan(),
        &answer_header.cyan()
    );

    let answer_file = File::open(&answer_path)?;
    let mut is_answer_start = false;
    for answer_line in BufReader::new(answer_file).lines()
    {
        let answer_line = answer_line?;

        //  回答の始まり
        if answer_line.to_lowercase()
            .ends_with(&("# ".to_string() + answer_header))
        {
            is_answer_start = true;
            continue;
        }

        //  回答の終わり
        if is_answer_start && answer_line.starts_with("#")
        {
            break;
        }

        //  空行スキップ
        if !is_answer_start || answer_line.len() <= 0 { continue; }

        //  回答の表示
        println!("\t\t{}", markdown_to_text::convert(&answer_line).cyan());
    }

    Ok(())
}

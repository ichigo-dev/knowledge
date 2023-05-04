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
    let challenge_upper = 3;

    //  ファイルを読み込む
    let file = File::open(doc_root.clone() + filename)
        .expect("file not found");
    let mut reader = EasyReader::new(file)
        .expect("something went wrong while reading the file");
    let _ = reader.build_index();

    let mut quiz_cnt =  0;
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

        //  問題用ファイルを開く
        let quiz_link = items[0];
        let quiz = quiz_link.replace("(./", "").replace(")", "");
        let quiz_paths = quiz.split("#").collect::<Vec<_>>();
        let quiz_path = doc_root.clone() + quiz_paths[0];
        let quiz_header = quiz_paths[1];
        let quiz_answer = items[1];

        let quiz_file = skip_fail!(File::open(&quiz_path));
        let mut is_quiz = false;
        let mut is_quiz_start = false;
        let mut print_quiz_header = String::new();
        let mut print_quiz_content = String::new();
        for quiz_line in BufReader::new(quiz_file).lines()
        {
            let quiz_line = skip_fail!(quiz_line);

            //  問題の始まり
            if quiz_line.to_lowercase()
                .ends_with(&("# ".to_string() + quiz_header))
            {
                quiz_cnt += 1;
                print_quiz_header = format!
                (
                    "{}{}{}\n",
                    " Q-".on_bright_cyan().black(),
                    quiz_cnt.to_string().on_bright_cyan().black(),
                    " ".on_bright_cyan().black()
                );
                is_quiz_start = true;
                continue;
            }

            //  問題の終わり
            if is_quiz_start && quiz_line.starts_with("#")
            {
                break;
            }

            //  空行スキップ
            if !is_quiz_start || quiz_line.len() <= 0 { continue; }

            //  問題の表示
            let quiz_content = markdown_to_text::convert(&quiz_line)
                .replace(quiz_answer, "\x1b[31m <???> \x1b[0m");

            if let Some(n) = quiz_content.find("<???>")
            {
                if n > 0 { is_quiz = true; }
            }
            print_quiz_content += &(format!("\t{}\n", quiz_content));
        }

        //  クイズを表示
        if is_quiz
        {
            println!("{}\n{}", print_quiz_header, print_quiz_content.cyan());
        }
        else
        {
            quiz_cnt -= 1;
            continue;
        }

        let mut fail_cnt = 0;
        let mut mask_pos = 0;
        loop
        {
            print!("Please input answer. [h(hint)/s(skip)/q(quit)] >> ");
            skip_fail!(std::io::stdout().flush());
            let mut result = String::new();
            skip_fail!(io::stdin().read_line(&mut result));

            match result.to_lowercase().trim()
            {
                s if s.len() >= 2 &&
                (
                    quiz_answer.to_lowercase().starts_with(s)
                    || quiz_answer.to_lowercase().ends_with(s)
                ) =>
                {
                    correct_cnt += 1;
                    println!("\t{}\n", "Good".bold().green());
                    break;
                },
                "h" =>
                {
                    mask_pos += 1;
                    print!
                    (
                        "\t{}{}{}",
                        "Hint-".yellow(),
                        mask_pos.to_string().yellow(),
                        ". ".yellow(),
                    );

                    for (i, char) in quiz_answer.chars().enumerate()
                    {
                        if i >= mask_pos
                        {
                            print!("X");
                        }
                        else
                        {
                            print!("{}", char.to_string().yellow());
                        }
                    }
                    print!("\n");

                    skip_fail!(std::io::stdout().flush());
                }
                "s" =>
                {
                    println!
                    (
                        "\t{}{}\n",
                        "Answer. ".bold().red(),
                        quiz_answer.bold().red()
                    );
                    break;
                }
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
                _ =>
                {
                    //  3回間違えたら回答を表示
                    fail_cnt += 1;
                    println!
                    (
                        "\t{}{}{}{}\n",
                        "Failed. ".bold().red(),
                        fail_cnt.to_string().bold().red(),
                        "/".bold().red(),
                        challenge_upper.to_string().bold().red()
                    );
                    if fail_cnt >= challenge_upper
                    {
                        println!
                        (
                            "\t{}{}\n",
                            "Answer. ".bold().red(),
                            quiz_answer.bold().red()
                        );
                        break;
                    }
                },
            }
        }
    }
}

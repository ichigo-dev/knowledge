use std::fs::File;
use std::io::{ self, Write, BufRead, BufReader };

use easy_reader::EasyReader;
use markdown_to_text;

fn main()
{
    let doc_root = String::from("note/ja/");
    let filename = "checksheet.md";

    //  ファイルを読み込む
    let file = File::open(doc_root.clone() + filename).unwrap();
    let mut reader = EasyReader::new(file).unwrap();
    let _ = reader.build_index();

    let mut question_cnt = 0;
    let mut ok_cnt = 0;
    loop
    {
        //  ランダムに1行取得して、空行かヘッダ行ならスキップ
        let line = reader.random_line().unwrap().unwrap().trim().to_string();
        if line.len() <= 0 || line.starts_with("#")
        {
            continue;
        }

        //  問題を出題
        question_cnt += 1;
        let items = line
            .rsplit(&['[', ']'])
            .collect::<Vec<_>>();
        println!("\nQ-{}. {}\n", question_cnt, &items[1]);

        //  回答を確認
        let mut hint_depth = 0;
        let mut hint_cnt = 0;
        'answer: loop
        {
            print!("\tCould you explain this term? [y/n/q(quit)] >> ");
            std::io::stdout().flush().unwrap();
            let mut result = String::new();
            io::stdin().read_line(&mut result).unwrap();

            match result.trim()
            {
                "y" =>
                {
                    ok_cnt += 1;
                    println!("\t\tGood!");
                    break;
                },
                "n" =>
                {
                    //  わからなければヒントを出す
                    hint_cnt += 1;
                    'hint: loop
                    {
                        //  前の行を探索してヘッダ行を探す
                        let prev_line = reader.prev_line().unwrap().unwrap()
                            .trim()
                            .to_string();
                        if prev_line.starts_with("#") == false
                        {
                            continue;
                        }

                        let header_depth = prev_line.rmatch_indices("#")
                            .collect::<Vec<_>>()
                            .len();
                        if hint_depth <= 0
                        {
                            hint_depth = header_depth + 1;
                        }

                        if header_depth < 2
                        {
                            //  ヒントがなくなったら回答を表示
                            let answer = items[0]
                                .replace("(./", "")
                                .replace(")", "");
                            let answer = answer
                                .split("#")
                                .collect::<Vec<_>>();
                            let answer_path = doc_root.clone() + answer[0];
                            let answer_header = answer[1];
                            println!
                            (
                                "\t\tAnswer. {} >> {}\n",
                                &answer_path,
                                &answer_header
                            );

                            //  回答ファイルの該当箇所を表示
                            let answer_file = File::open(&answer_path).unwrap();
                            let mut answer_start = false;
                            for answer_line in BufReader::new(answer_file).lines()
                            {
                                let answer_line = answer_line.unwrap();

                                //  回答の終わり
                                if answer_line.starts_with("#") && answer_start
                                {
                                    break;
                                }

                                //  回答の始まり
                                if answer_line.to_lowercase().ends_with(answer_header)
                                {
                                    answer_start = true;
                                    continue;
                                }

                                //  回答が始まっていなければスキップ
                                if answer_start == false
                                    || answer_line.len() <= 0
                                {
                                    continue;
                                }

                                println!
                                (
                                    "\t\t\t{}",
                                    markdown_to_text::convert(&answer_line)
                                );
                            }
                            break 'answer;
                        }
                        else if hint_depth > header_depth
                        {
                            //  次のヒントを表示
                            hint_depth = header_depth;
                            println!
                            (
                                "\t\tHint-{}. {}\n",
                                hint_cnt,
                                prev_line.trim_start_matches("#").trim()
                            );
                            break 'hint;
                        }
                    }
                },
                "q" =>
                {
                    println!("\nResult: {}/{}", ok_cnt, question_cnt-1);
                    return;
                },
                _ => {},
            }
        }
    }
}

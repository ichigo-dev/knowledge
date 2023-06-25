/*

    A GUI tool that automatically generates quizes from knowledge notes.

*/

use std::path::{ Path, PathBuf };
use std::fs::{ self, File };
use std::io::Write;
use std::fmt;

use clap::Parser;
use chrono::Local;
use markdown::{ self, Block, Span, ListItem };
use once_cell::sync::OnceCell;
use rand::seq::SliceRandom;
use regex::Regex;
use serde::{ Serialize, Deserialize };
use serde_json;
use termion::{ color, style };

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli
{
    //  Path to the directory containing knowledge notes.
    #[clap
    (
        short='n',
        long="note-path",
        value_name="PATH",
        default_value="./note/ja"
    )]
    note_path: String,

    //  Path to the checksheet.
    #[clap
    (
        short='c',
        long="checksheet",
        value_name="PATH",
        default_value="./note/ja/checksheet.md"
    )]
    checksheet_path: String,

    //  Path to output the Learning log.
    #[clap
    (
        short='o',
        long="output",
        value_name="PATH",
        default_value="./"
    )]
    output_path: String,
}

//  Static log file.
static LOG_FILE: OnceCell<File> = OnceCell::new();

fn main()
{
    let cli = Cli::parse();

    //  Creates a log file.
    let mut output_path = PathBuf::from(cli.output_path);
    output_path.push
    (
        format!("learning_{}.log", Local::now().format("%Y%m%d%H%M%S"))
    );
    let log_file = File::create(output_path)
        .expect("failed to create log file");
    LOG_FILE.set(log_file).unwrap();

    //  Parses the checksheet and convert it to a vector of terms.
    let mut terms = if Path::new("terms.cache").is_file()
    {
        load_terms_cache()
    }
    else
    {
        let checksheet = fs::read_to_string(cli.checksheet_path)
            .expect("failed to read checksheet");
        let terms = parse_checksheet(&checksheet);
        create_terms_cache(&terms);
        terms
    };

    //  Shuffles the terms.
    terms.shuffle(&mut rand::thread_rng());

    //  Generates a quiz for each term.
    let mut result = Result
    {
        number_of_quiz: 0,
        correct: 0,
        incorrect_terms: Vec::new(),
    };
    for term in terms
    {
        result.number_of_quiz += 1;
        output!
        (
            "{}{} Q-{} {}\n\n",
            color::Bg(color::Blue),
            color::Fg(color::Black),
            result.number_of_quiz,
            style::Reset
        );
        quiz(&term, &mut result);
        output!("\n\n");
    }
}

//------------------------------------------------------------------------------
//  A structure 
//------------------------------------------------------------------------------
#[derive(Debug)]
struct Result
{
    number_of_quiz: usize,
    correct: usize,
    incorrect_terms: Vec<Term>,
}

impl fmt::Display for Result
{
    fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
    {
        let mut incorrect_terms = String::new();
        for term in &self.incorrect_terms
        {
            incorrect_terms.push_str(&format!
            (
                "\n* {}: {}",
                term.term,
                term.link
            ));
        }

        write!
        (
            f,
            "{}{}Result: {}/{} ({}%){}\n{}{}{}{}",
            style::Bold,
            color::Fg(color::Green),
            self.correct, self.number_of_quiz,
            self.correct as f32 / self.number_of_quiz as f32 * 100.0,
            style::Reset,
            style::Bold,
            color::Fg(color::Red),
            incorrect_terms,
            style::Reset
        )
    }
}

//------------------------------------------------------------------------------
//  A structure representing a term.
//------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Term
{
    //  Term.
    term: String,

    //  Path to the note.
    link: String,

    //  Path to the fragment in the note.
    flagment: String,

    //  Explanation of the term.
    content: String,
}


//------------------------------------------------------------------------------
//  Parses the checksheet and convert it to a vector of terms.
//
//  The terms writen on the checksheet have the following format:
//  - [term](link)
//
//  The link is the path to the note and the content is written in there.
//------------------------------------------------------------------------------
fn parse_checksheet( checksheet: &str ) -> Vec<Term>
{
    let mut terms = Vec::new();

    for line in checksheet.lines()
    {
        if let Some(term) = parse_checksheet_line(line)
        {
            terms.push(term);
        }
    }

    terms
}

//  Parses a line of checksheet and convert it to a term.
fn parse_checksheet_line( line: &str ) -> Option<Term>
{
    //  Gets the part enclosed by `[]` and `()` from the line.
    let parts = line
        .split(|c| c == '[' || c == ']' || c == '(' || c == ')')
        .map(|s| s.trim())
        .filter(|s|
            s.is_empty() == false
            && s != &"-"
            && s.starts_with(&"#") == false
        )
        .collect::<Vec<&str>>();

    //  If line contains a term, it should be of the form `[term, link]`.
    if parts.len() != 2
    {
        return None;
    }

    let term = parts[0];
    let (link, flagment) = if parts[1].contains('#')
    {
        //  If the link contains a flagment, splits it.
        let parts = parts[1].split_once('#').unwrap();
        (convert_path(parts.0), parts.1)
    }
    else
    {
        (convert_path(parts[1]), "")
    };

    //  Extracts a specific fragment from the note.
    let content = fs::read_to_string(link.clone()).unwrap_or("".to_string());
    let content = if flagment.is_empty()
    {
        content
    }
    else
    {
        //  Extracts a section from the note.
        let mut section = String::new();
        let mut is_in_section = false;
        for line in content.lines()
        {
            if is_in_section
            {
                if line.starts_with("#")
                {
                    break;
                }
                section.push_str(&(line.to_owned() + "\n"));
            }
            else if line.to_lowercase().contains(&("# ".to_string() + flagment))
            {
                is_in_section = true;
            }
        }

        section
    };

    Some(Term
    {
        term: term.to_string(),
        link: link,
        flagment: flagment.to_string(),
        content: content,
    })
}

//------------------------------------------------------------------------------
//  Converts path to path from base path.
//------------------------------------------------------------------------------
fn convert_path( path: &str ) -> String
{
    let cli = Cli::parse();
    let mut note_path = PathBuf::from(cli.note_path.clone());
    note_path.push(path);
    note_path.into_os_string().into_string().unwrap_or("".to_string())
}

//------------------------------------------------------------------------------
//  Creates a cache file for each term.
//------------------------------------------------------------------------------
fn create_terms_cache( terms: &[Term] )
{
    let json_string = serde_json::to_string(terms).unwrap();
    let mut cache_file = File::create("terms.cache").unwrap();
    let _ = cache_file.write_all(json_string.as_bytes()).unwrap();
}

//------------------------------------------------------------------------------
//  Load a cache file for each term.
//------------------------------------------------------------------------------
fn load_terms_cache() -> Vec<Term>
{
    let content = fs::read_to_string("terms.cache").unwrap();
    serde_json::from_str(&content).unwrap()
}

//------------------------------------------------------------------------------
//  Generates a quiz for a term. Returns true if the answer is correct.
//------------------------------------------------------------------------------
fn quiz( term: &Term, result: &mut Result )
{
    let quiz = generate_quiz(term);
    print_quiz(quiz);

    let max_retry_cnt = 3;
    let mut retry_cnt = 0;
    let mut hint_cnt = 0;
    loop
    {
        output!("Please input answer. [h(hint)/s(skip)/q(quit)] >> ");
        std::io::stdout().flush().unwrap();
        match get_input()
        {
            s if s == "h".to_string() =>
            {
                hint_cnt += 1;
                if hint_cnt == term.term.chars().count()
                {
                    outputln!("No more hints.");
                    outputln!
                    (
                        "{}Answer: {}{}",
                        color::Fg(color::Red),
                        term.term,
                        style::Reset
                    );
                    return;
                }

                output!("{}", color::Fg(color::Yellow));
                for (i, char) in term.term.chars().enumerate()
                {
                    if i < hint_cnt
                    {
                        output!("{}", char);
                    }
                    else
                    {
                        output!("_");
                    }
                }
                output!("{}\n", style::Reset);
            },
            s if s == "s".to_string() =>
            {
                outputln!("Skip this quiz.");
                outputln!
                (
                    "{}Answer: {}{}",
                    color::Fg(color::Red),
                    term.term,
                    style::Reset
                );
                result.number_of_quiz -= 1;
                return;
            },
            s if s == "q".to_string() =>
            {
                result.number_of_quiz -= 1;
                quit_quiz(result);
            },
            s =>
            {
                let lower_s = s.trim().to_lowercase();
                let lower_term = term.term.trim().to_lowercase();
                if lower_s == lower_term
                    || lower_s.len() >= 5 && lower_term.contains(&lower_s)
                {
                    outputln!("Correct!");
                    outputln!
                    (
                        "{}Answer: {}{}",
                        color::Fg(color::Green),
                        term.term,
                        style::Reset
                    );
                    result.correct += 1;
                    return;
                }
                else
                {
                    retry_cnt += 1;
                    if retry_cnt >= max_retry_cnt
                    {
                        outputln!("Failed...");
                        outputln!
                        (
                            "{}Answer: {}{}",
                            color::Fg(color::Red),
                            term.term,
                            style::Reset
                        );
                        result.incorrect_terms.push(term.clone());
                        return;
                    }

                    outputln!("Retry({}/{})", retry_cnt, max_retry_cnt);
                }
            },
        }
    }
}

//  Generates a quiz for a term.
fn generate_quiz( term: &Term ) -> String
{
    let mut content = term.content.clone();
    let target = &term.term;
    let replace = &format!("<xxxxx>");

    //  Masks the term and alias.
    content = content.replace(['*', '`'], "");
    content = content.replace(target, replace);
    let re = Regex::new(r"\(([^\)]*)\)|（([^）]*)）").unwrap();
    for cap in re.captures_iter(&target)
    {
        let mut alias = String::new();
        if let Some(s) = cap.get(1)
        {
            alias = s.as_str().to_string();
        }
        else if let Some(s) = cap.get(2)
        {
            alias = s.as_str().to_string();
        }

        if alias.is_empty() == false
        {
            content = content.replace(&alias, replace);
            break;
        }
    }

    content.trim().to_string()
}

//  Prints a quiz.
fn print_quiz( quiz: String )
{
    let tokens = markdown::tokenize(&quiz);
    for token in tokens
    {
        print_block(token);
    }
    output!("\n\n");
}

fn print_block( block: Block )
{
    match block
    {
        Block::Header(v, _) =>
        {
            output!("{}", color::Bg(color::Cyan));
            for span in v
            {
                print_span(span);
            }
            output!("{}", style::Reset);
        },
        Block::Paragraph(v) =>
        {
            for span in v
            {
                print_span(span);
            }
        },
        Block::Blockquote(v) =>
        {
            for block in v
            {
                print_block(block);
            }
        },
        Block::CodeBlock(s1, s2) =>
        {
            outputln!
            (
                "\n=====\n{}{}{}{}\n=====\n",
                color::Bg(color::LightBlack),
                s1.unwrap_or("".to_string()),
                s2,
                style::Reset
            );
        },
        Block::OrderedList(v, _) =>
        {
            for (index, item) in v.into_iter().enumerate()
            {
                output!("\n{}. ", index + 1);
                print_item(item);
            }
            output!("\n");
        },
        Block::UnorderedList(v) =>
        {
            for item in v
            {
                output!("\n* ");
                print_item(item);
            }
            output!("\n");
        },
        Block::Raw(s) =>
        {
            outputln!("{}", s);
        },
        Block::Hr =>
        {
            outputln!("{}", std::iter::repeat("-").take(80).collect::<String>());
        },
    }
}

fn print_span( span: Span )
{
    match span
    {
        Span::Break => { output!("\n"); },
        Span::Text(s) =>
        {
            if s.starts_with('|')
            {
                output!("\n{}", s);
            }
            else
            {
                output!("{}", s);
            }
        }
        Span::Code(s) =>
        {
            output!("{}{}{}", color::Bg(color::LightBlack), s, style::Reset);
        },
        Span::Link(s, _, _) =>
        {
            output!
            (
                "{}{}{}{}",
                color::Fg(color::Blue),
                style::Underline,
                s,
                style::Reset
            );
        }
        Span::Emphasis(v) =>
        {
            output!("{}", style::Italic);
            for span in v
            {
                print_span(span);
            }
            output!("{}", style::Reset);
        },
        Span::Strong(v) =>
        {
            output!("{}", style::Bold);
            for span in v
            {
                print_span(span);
            }
            output!("{}", style::Reset);
        },
        _ => {},
    }
}

fn print_item( item: ListItem )
{
    match item
    {
        ListItem::Simple(v) =>
        {
            for span in v
            {
                print_span(span);
            }
        },
        ListItem::Paragraph(v) =>
        {
            for block in v
            {
                print_block(block);
            }
        },
    }
}

//------------------------------------------------------------------------------
//  Gets user input.
//------------------------------------------------------------------------------
fn get_input() -> String
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

//------------------------------------------------------------------------------
//  Quits the quiz.
//------------------------------------------------------------------------------
fn quit_quiz( result: &Result )
{
    if result.number_of_quiz > 0
    {
        outputln!("{}", result);
    }
    std::process::exit(0);
}

//------------------------------------------------------------------------------
//  Macro for output message.
//------------------------------------------------------------------------------
#[macro_export]
macro_rules! output
{
    ($($arg:tt)*) =>
    {
        let mut s = String::new();
        std::fmt::write(&mut s, format_args!($($arg)*)).unwrap();
        print!("{}", s);

        let mut log_file = LOG_FILE.get().unwrap();
        let _ = write!(log_file, "{}", s);
    };
}

#[macro_export]
macro_rules! outputln
{
    ($($arg:tt)*) =>
    {
        let mut s = String::new();
        std::fmt::write(&mut s, format_args!($($arg)*)).unwrap();
        println!("{}", s);

        let mut log_file = LOG_FILE.get().unwrap();
        let _ = writeln!(log_file, "{}", s);
    };
}

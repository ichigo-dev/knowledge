/*

    A CLI tool that automatically generates quizes from knowledge notes.

*/

use std::path::PathBuf;
use std::fs;

use clap::Parser;

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

fn main()
{
    let cli = Cli::parse();

    let checksheet = fs::read_to_string(cli.checksheet_path)
        .expect("failed to read checksheet");
    let terms = parse_checksheet(&checksheet);
}


//------------------------------------------------------------------------------
//  A structure representing a term.
//------------------------------------------------------------------------------
#[derive(Debug)]
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
        let mut parts = parts[1].split_once('#').unwrap();
        (convert_path(parts.0), parts.1)
    }
    else
    {
        (convert_path(parts[1]), "")
    };

    //  Extracts a specific fragment from the note.
    let content = fs::read_to_string(link).unwrap_or("".to_string());
    let content = if flagment.is_empty()
    {
        content
    }
    else
    {
        //  Splits sections by lines startnig with `#`.
        let mut sections = Vec::new();
        let mut start = 0;
        let mut is_in_section = false;
        for (index, line) in content.lines().enumerate()
        {
            if line.starts_with("#")
            {
                if is_in_section
                {
                    let section = &content[start..index];
                    sections.push(section.trim());
                    is_in_section = false;
                }

                start = index;
                is_in_section = true;
            }
        }
        println!("{:?}", sections);

        "".to_string()
    };

    Some(Term
    {
        term: String::new(),
        link: String::new(),
        flagment: String::new(),
        content: String::new(),
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

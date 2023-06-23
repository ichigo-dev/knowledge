/*

    A CLI tool that automatically generates quizes from knowledge notes.

*/

use std::path::{ Path, PathBuf };
use std::fs::{ self, File };
use std::io::Write;

use clap::Parser;
use serde::{ Serialize, Deserialize };
use serde_json;
use rand::seq::SliceRandom;

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
    for (index, term) in terms.enumerate()
    {
        println!("Q-{}", index + 1);
        quiz(term);
    }
}


//------------------------------------------------------------------------------
//  A structure representing a term.
//------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
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

                section.push_str(line);
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
//  Generates a quiz for a term.
//------------------------------------------------------------------------------
fn quiz( term: Term )
{
    println!("{}");
}

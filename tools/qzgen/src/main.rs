use std::path::{ Path, PathBuf };
use std::fs::{ self, File };
use std::io::Write;

use clap::Parser;
use chrono::Utc;
use glob::glob;
use markdown::{ self, Span, Block };

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli
{
    #[clap
    (
        short='n',
        long="note",
        value_name="PATH",
        default_value="./note/ja"
    )]
    note_path: String,

    #[clap
    (
        short='c',
        long="checksheet",
        value_name="PATH",
        default_value="./note/ja/checksheet.md"
    )]
    checksheet: String,

    #[clap
    (
        long="no-report",
        default_value="false"
    )]
    no_report: bool,

    #[clap
    (
        short='o',
        long="output",
        value_name="PATH",
        default_value="./"
    )]
    output: String,
}

fn main()
{
    let cli = Cli::parse();
    let note_path = PathBuf::from(&cli.note_path);
    let output_path = PathBuf::from(&cli.output);
    let checksheet = fs::read_to_string(cli.checksheet)
        .expect("failed to read checksheet");
    let terms = parse_checksheet(&checksheet);

    if cli.no_report == false
    {
        report(output_path.clone(), note_path.clone());
    }
}

//  Get file path
fn get_file_path( mut output_path: PathBuf, file_prefix: &str ) -> PathBuf
{
    output_path
        .push(format!
        (
            "{}_{}.log",
            file_prefix,
            Utc::now().format("%Y%m%d_%H%M%S")
        ));
    output_path
}

//  Parse checksheet to terms.
//
//  Header parts (like `# Heading`) are skipped. Formats like
//  `- [ ] [term](link)` are allowed for terms. Other formats may cause
//  problems.
fn parse_checksheet( checksheet: &str ) -> Vec<(String, String)>
{
    let mut counter = 0;
    let lines = checksheet.lines();
    lines.clone().for_each(|_| counter += 1);
    let mut result = Vec::with_capacity(counter);

    for line in lines
    {
        if line.len() <= 0 || line.starts_with('#')
        {
            continue;
        }

        let items = line
            .rsplit(&['[', ']'])
            .collect::<Vec<_>>();
        if items.len() <= 2 || items[items.len() - 1].trim() != "-"
        {
            continue;
        }

        let term = items[1].to_string();
        let link = items[0].replace(['(', ')'], "");
        result.push((term, link));
    }

    result
}

//  Output report.
fn report( output_path: PathBuf, note_path: PathBuf )
{
    let report_file_path = get_file_path(output_path, "report");
    let mut report_file = File::create(report_file_path)
        .expect("failed to create a report file");

    let mut note_path = note_path.clone();
    note_path.push("**/*.md");
    let files = glob(&note_path.into_os_string().into_string().unwrap())
        .unwrap()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    for file in files
    {
        let content = fs::read_to_string(file).unwrap();
        let tokens = markdown::tokenize(&content);

        for token in tokens
        {
        }
    }
}

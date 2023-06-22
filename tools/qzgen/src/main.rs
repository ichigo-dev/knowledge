use std::path::Path;
use std::fs::{ self, File };
use std::io::{ self, BufRead, Write, BufReader };

use clap::Parser;
use markdown::{ tokenize, Block };

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli
{
    #[clap(short='c', long="checksheet", value_name="PATH", default_value="./note/ja/checksheet.md")]
    checksheet: String,

    #[clap(long="no-report", default_value="false")]
    no_report: bool,

    #[clap(short='o', long="output", value_name="PATH", default_value="./")]
    output: String,
}

fn main()
{
    let cli = Cli::parse();
    let checksheet = fs::read_to_string(cli.checksheet)
        .expect("aailed to read checksheet");
    let terms = parse_checksheet(&checksheet);

    if cli.no_report == false
    {
        report();
    }
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
fn report()
{
}

use std::path::{ Path, PathBuf };
use std::fs::{ self, File };
use std::io::Write;

use clap::Parser;
use chrono::Utc;
use glob::glob;
use reqwest;
use rand::seq::SliceRandom;
use markdown_to_text;

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
        long="report",
        default_value="false"
    )]
    report: bool,

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
    let mut terms = parse_checksheet(&checksheet, note_path.clone());
    let mut rng = rand::thread_rng();
    terms.shuffle(&mut rng);

    //  Output report.
    if cli.report
    {
        report(output_path.clone(), note_path.clone(), &terms);
    }

    for term in terms
    {
        //println!("{} link: {} content: {}", term.name, term.link, term.content);
    }
}

//  Get the file path.
fn get_file_path( mut output_path: PathBuf, file_prefix: &str ) -> PathBuf
{
    output_path
        .push(format!
        (
            "{}_{}.log",
            file_prefix,
            Utc::now().format("%Y%m%d_%I%M%S")
        ));
    output_path
}

struct Term
{
    name: String,
    link: String,
    content: String,
}

//  Parse checksheet to terms.
//
//  Header parts (like `# Heading`) are skipped. Formats like
//  `- [ ] [term](link)` are allowed for terms. Other formats may cause
//  problems.
fn parse_checksheet( checksheet: &str, filepath: PathBuf ) -> Vec<Term>
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

        let name = items[1].to_string();
        let mut link = items[0].replace(['(', ')'], "");
        let mut content = String::new();

        //  Get content
        if let Some((link_part, title)) = link.clone().split_once('#')
        {
            let link_path = filepath.join(Path::new(link_part));
            link = link_path.to_str().unwrap().to_string();
            if link_path.exists() && link_path.is_file()
            {
                let target = "# ".to_string() + &title.to_lowercase();
                let input = fs::read_to_string(link_path).unwrap();
                if let Some(start) = input.find(&target)
                {
                    content = input[(start + target.len())..].to_string();
                    if let Some(end) = content.find("# ")
                    {
                        content = content[..(end - 2)].to_string();
                    }
                }
            }
        }

        let term = Term
        {
            name,
            link: link.to_string(),
            content: markdown_to_text::convert(content.trim()),
        };
        result.push(term);
    }

    result
}

//  Output report.
fn report( output_path: PathBuf, note_path: PathBuf, terms: &[Term] )
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

    //  Check for broken links
    let link_regex = regex::Regex::new(r"\[.*?\]\(([^)\s]+)\)").unwrap();
    for file in files
    {
        let content = fs::read_to_string(file.clone()).unwrap();

        for (n, line) in content.lines().enumerate()
        {
            for capture in link_regex.captures_iter(line)
            {
                if let Some(link) = capture.get(1)
                {
                    let link = link.as_str();
                    if link.starts_with('#')
                    {
                        continue;
                    }

                    if check_link(link, file.clone()) == false
                    {
                        let log = format!
                        (
                            "[{}:{}] link error: {}",
                            file.as_os_str().to_str().unwrap(),
                            n + 1,
                            link
                        );
                        println!("{}", log);
                        let _ = writeln!(report_file, "{}", log);
                    }
                }
            }
        }
    }

    //  Check if sentences can be a quiz.
    for term in terms
    {
        let name = &term.name;
        let content = &term.content;
        if content.contains(name) == false
        {
           let log = format!
            (
                "the term '{}' is not contained in the '{}'",
                name,
                content,
            );
            println!("{}", log);
            let _ = writeln!(report_file, "{}", log);
        }
    }
}

fn check_link( link: &str, filepath: PathBuf ) -> bool
{
    if link.starts_with("http")
    {
        let response = reqwest::blocking::get(link);
        if let Ok(res) = response
        {
            return res.status().is_success();
        }
    }

    let base_path = filepath.parent().unwrap();
    if let Some((link, title)) = link.split_once('#')
    {
        let link_path = base_path.join(Path::new(link));
        if link_path.exists() && link_path.is_file()
        {
            let content = fs::read_to_string(link_path).unwrap();
            let target = "# ".to_string() + &title.to_lowercase();
            return content.to_lowercase().contains(&target);
        }
    }
    else
    {
        let link_path = base_path.join(Path::new(link));
        return link_path.exists();
    }

    return false;
}

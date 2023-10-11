//------------------------------------------------------------------------------
//! Read data source and generate quiz.
//------------------------------------------------------------------------------

mod data_source;
mod converter;
mod term;

use data_source::{ DataSource, FileDataSource };
use converter::Converter;
use term::Dictionary;

use glob::glob;

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
fn main()
{
    let glob = glob("../../note/**/*.md")
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
                return;
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

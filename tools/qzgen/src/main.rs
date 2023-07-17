/*

    Application endpoint.

*/

mod component;
mod page;
mod app;
mod theme;
mod data;

pub use data::Term;
use app::App;

const API_URL: &str = "https://y31vtnaik7.execute-api.ap-northeast-1.amazonaws.com";
const NOTE_URL: &str = "https://github.com/ichigo-dev/knowledge/blob/main";
const NOTE_PATH_PREFIX: &str = "./knowledge-main";
const MAX_TRY_CNT: usize = 3;

fn main()
{
    console_log::init_with_level(log::Level::Info).unwrap();
    sycamore::render(App);
}

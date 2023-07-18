/*

    Application endpoint.

*/

mod component;
mod page;
mod app;
mod theme;
mod data;
mod functions;

pub use data::*;
use app::*;

const API_URL: &str = "https://y31vtnaik7.execute-api.ap-northeast-1.amazonaws.com";
const NOTE_URL: &str = "https://github.com/ichigo-dev/knowledge/blob/main";
const NOTE_PATH_PREFIX: &str = "./knowledge-main";
const MAX_TRY_CNT: usize = 3;
const USER_CODE_KEY: &str = "user_code";

fn main()
{
    console_log::init_with_level(log::Level::Info).unwrap();
    sycamore::render(App);
}

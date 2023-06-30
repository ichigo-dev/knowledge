mod app;
mod common;
mod panel;
mod theme;

use app::App;

fn main()
{
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}

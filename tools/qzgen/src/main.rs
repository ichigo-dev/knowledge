mod app;
mod component;
mod panel;
mod theme;
mod language;
mod term;
mod api;

use app::App;

fn main()
{
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}

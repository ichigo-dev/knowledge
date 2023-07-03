mod app;
mod component;
mod panel;
mod theme;
mod language;

use app::App;

fn main()
{
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}

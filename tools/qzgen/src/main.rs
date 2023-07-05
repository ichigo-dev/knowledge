mod app;
mod component;
mod theme;

use app::App;

fn main()
{
    console_log::init_with_level(log::Level::Info).unwrap();
    sycamore::render(App);
}

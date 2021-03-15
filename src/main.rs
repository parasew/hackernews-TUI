// modules
pub mod hn_client;
pub mod prelude;
pub mod view;

use prelude::*;

fn main() {
    env_logger::init();

    let mut siv = cursive::default();

    // load theme
    siv.load_toml(include_str!("../theme.toml")).unwrap();

    let client = hn_client::HNClient::new().unwrap();
    let async_view = async_view::get_front_page_story_view_async(&mut siv, &client);
    siv.screen_mut()
        .add_transparent_layer(Layer::new(async_view));

    siv.add_global_callback(Event::AltChar('q'), |s| s.quit());

    siv.run();
}

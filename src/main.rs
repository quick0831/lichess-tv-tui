use std::sync::mpsc::sync_channel;

use color_eyre::Result;

mod app;
use app::App;

mod api;
use api::get_lichess_tv;

fn main() -> Result<()> {
    set_panic_hook();
    color_eyre::install()?;
    let (tx, rx) = sync_channel(0);
    std::thread::spawn(|| get_lichess_tv(tx));
    let mut terminal = ratatui::init();
    let app_result = App::new(rx).run(&mut terminal);
    ratatui::restore();
    app_result
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        hook(panic_info);
    }));
}

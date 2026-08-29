use crate::app::App;

pub mod app;
// pub mod components;
// pub mod config;
pub mod datamodel;
pub mod editor;
pub mod event;
pub mod persistence;
pub mod test_config;
pub mod ui;
pub mod utils;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    if let Ok(str) = result {
        println!("{}", str);
        Ok(())
    } else {
        result.map(|_| ())
    }
}

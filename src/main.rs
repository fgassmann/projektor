use std::process::{ExitCode, exit};

use crate::app::App;

pub mod app;
pub mod cli;
pub mod datamodel;
pub mod editor;
pub mod event;
pub mod persistence;
pub mod ui;
pub mod utils;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = App::new().and_then(|app| app.run(terminal));
    ratatui::restore();
    match result {
        Ok(Some(str)) => {
            println!("{}", str);
            Ok(())
        }
        Ok(None) => exit(3),
        _ => result.map(|_| ()),
    }
}

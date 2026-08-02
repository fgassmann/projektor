use crate::event::{AppEvent, Event, EventHandler};

use crate::config::{self};
use crate::test_config::get_test_config;
use color_eyre::eyre::eyre;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::KeyEvent;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,

    pub tab: Tab,
    pub popups: Vec<Popup>,
    pub config: config::Config,
}

#[derive(Debug)]
pub enum Popup {
    Error(String),
}

#[derive(Debug)]
pub enum Tab {
    Projects,
    Templates,
    Settings,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),

            tab: Tab::Projects,
            popups: Vec::new(),
            config: get_test_config(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<String> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        if let Some((_, p)) = self.config.projects.get_from_rendered() {
            return Ok(p.path.to_string_lossy().to_string());
        }
        Err(eyre!("Nothing Selected"))
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => {} //self.tick(),
            Event::Crossterm(event) => match event {
                ratatui::crossterm::event::Event::Key(key_event)
                    if key_event.kind == ratatui::crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                AppEvent::Save => self.config.save(),
                _ => {}
            },
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if !self.popups.is_empty() {
            self.popups.pop();
        }
        match &mut self.tab {
            Tab::Projects => {
                if let Some(e) = self
                    .config
                    .projects
                    .handle_key_event(key_event)
                    .unwrap_or_else(|e| {
                        self.popups.push(Popup::Error(e.to_string()));
                        None
                    })
                {
                    self.events.send(e);
                }
            }
            _ => {}
        }
        Ok(())
    }

    // fn tick(&self) {}

    fn quit(&mut self) {
        self.running = false;
    }
}

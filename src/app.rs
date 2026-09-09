use crate::cli;
use crate::event::{AppEvent, Event, EventHandler};

use crate::datamodel::ProjectListView;
use clap::Parser;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::KeyEvent;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,

    pub tab: Tab,
    pub popups: Vec<Popup>,
    pub data: ProjectListView,
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

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let args = cli::Args::parse();
        Ok(Self {
            running: true,
            events: EventHandler::new(),

            tab: Tab::Projects,
            popups: Vec::new(),
            data: ProjectListView::new(&args)?,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<Option<String>> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        if let Some((_, p)) = self.data.get_from_rendered() {
            return Ok(Some(p.path.to_string_lossy().to_string()));
        }
        Ok(None)
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => {}
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
                AppEvent::Save => self.handle_err(self.data.projects.save()),
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
                if let Some(e) = self.data.handle_key_event(key_event).unwrap_or_else(|e| {
                    self.popups.push(Popup::Error(e.to_string()));
                    None
                }) {
                    self.events.send(e);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_err(&mut self, result: color_eyre::Result<()>) {
        if let Err(e) = result {
            self.popups.push(Popup::Error(e.to_string()));
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

use crate::cli;
use crate::config::{Config, Settings};
use crate::event::{AppEvent, Event, EventHandler};

use crate::datamodel::ProjectListView;
use clap::Parser;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::KeyEvent;

/// Application.
#[derive(Debug)]
pub struct App {
    pub state: AppState,
    pub events: EventHandler,

    pub tab: Tab,
    pub popups: Vec<Popup>,
    pub projects: ProjectListView,
    pub settings: Settings,
}

#[derive(Debug, Default)]
pub enum AppState {
    #[default]
    Running,
    Exit,
    Result(String),
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
        let Config {
            categories,
            settings,
        } = Config::load(&args.config)?;
        Ok(Self {
            state: AppState::Running,
            events: EventHandler::new(),

            tab: Tab::Projects,
            popups: Vec::new(),
            projects: ProjectListView::new(categories),
            settings,
        })
    }
    pub fn save(&self) -> color_eyre::Result<()> {
        let cfg = Config {
            categories: self.projects.categories.clone(),
            settings: self.settings.clone(),
        };
        cfg.save()?;
        Ok(())
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<Option<String>> {
        while matches!(self.state, AppState::Running) {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        if let AppState::Result(p) = self.state {
            Ok(Some(p))
        } else {
            Ok(None)
        }
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
                // TODO Fix this:
                AppEvent::Quit => self.state = AppState::Exit,
                AppEvent::QuitWithSelected => {
                    if let Some((_, p)) = self.projects.get_from_rendered() {
                        self.state = AppState::Result(p.path.to_string_lossy().to_string());
                    } else {
                        self.state = AppState::Exit;
                    }
                }
                AppEvent::Save => self.save().unwrap_or_else(|e| {
                    self.error_popup(e);
                }),
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
                    .projects
                    .handle_key_event(key_event, &self.settings)
                    .unwrap_or_else(|e| {
                        self.error_popup(e);
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

    fn error_popup(&mut self, err: color_eyre::Report) {
        self.popups.push(Popup::Error(err.to_string()));
    }
}

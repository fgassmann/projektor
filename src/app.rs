use crate::event::{AppEvent, Event, EventHandler};

use crate::config::{self, get_test_config};
use crate::editor;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,

    pub mode: AppMode,
    pub config: config::Config,
}

#[derive(Debug)]
pub enum AppMode {
    ProjectView,
    EditingProject(editor::ProjectEditor),
    CreatingProject(editor::ProjectEditor),
    Error(String, Box<AppMode>),
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),

            mode: AppMode::ProjectView,
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
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
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
                AppEvent::Down => self.config.projects.next(),
                AppEvent::Up => self.config.projects.prev(),
                AppEvent::Quit => self.quit(),

                AppEvent::Edit => self.edit_project(),
                AppEvent::New => self.new_project(),
                AppEvent::SaveEdit => self.save_project(),
                AppEvent::CancelEdit => self.mode = AppMode::ProjectView,
            },
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match &mut self.mode {
            AppMode::ProjectView => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q' | 'Q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
                }
                KeyCode::Char('j' | 'J') | KeyCode::Down => self.events.send(AppEvent::Down),
                KeyCode::Char('k' | 'K') | KeyCode::Up => self.events.send(AppEvent::Up),
                KeyCode::Char('e' | 'E') => {
                    self.events.send(AppEvent::Edit);
                }
                KeyCode::Char('n' | 'N') => {
                    self.events.send(AppEvent::New);
                }
                KeyCode::Char('d' | 'D') => todo!("Delete Currently selected"),
                _ => {}
            },
            AppMode::EditingProject(editor) | AppMode::CreatingProject(editor) => {
                if let Some(e) = editor.handle_key_event(key_event)? {
                    self.events.send(e);
                }
            }
            AppMode::Error(_, prev) => {
                self.mode = std::mem::replace(prev, AppMode::ProjectView);
            }
        }
        Ok(())
    }

    // fn tick(&self) {}

    fn quit(&mut self) {
        self.running = false;
    }

    fn new_project(&mut self) {
        self.mode = AppMode::CreatingProject(editor::ProjectEditor::default());
    }

    fn save_project(&mut self) {
        let mode = std::mem::replace(&mut self.mode, AppMode::ProjectView);
        match mode {
            AppMode::EditingProject(p) => {
                let (cat, proj) = p.into();
                self.config.projects.update_selected(proj, cat);
            }
            AppMode::CreatingProject(p) => {
                let (cat, proj) = p.into();
                self.config.projects.insert(proj, cat);
            }
            _ => {
                unreachable!(
                    "Error: trying to save while not creating or editing a project. This should not happen."
                )
            }
        }
        self.config.save();
    }
    fn edit_project(&mut self) {
        if let Some(p) = self.config.projects.get_from_rendered() {
            self.mode = AppMode::EditingProject(editor::ProjectEditor::from(p))
        } else {
            self.mode = AppMode::Error(
                String::from("Nothing selected"),
                Box::new(AppMode::ProjectView),
            )
        }
    }
}

use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::prelude::{Buffer, Constraint, Line, Rect, Style, Widget};
use ratatui::style::Modifier;
use ratatui::widgets::{Block, BorderType, Clear, WidgetRef};
use ratatui_textarea::{CursorMove, Input, TextArea, WrapMode};

// use crate::error;
use crate::utils::{THEME, explorer_theme};
use ratatui_explorer::{FileExplorer, FileExplorerBuilder};
use std::env;

use std::fs;
use std::path::PathBuf;
use std::string::ToString;

#[derive(Debug)]
enum PathInputMode {
    Normal,
    Explorer,
    Creating(SingleLineInput),
    Error(String),
}

#[derive(Debug)]
pub struct PathInput {
    mode: PathInputMode,
    original: PathBuf,
    input: FileExplorer,
}

impl PathInput {
    /// Returns true if it handled the event.
    pub fn input(&mut self, key_event: KeyEvent) -> bool {
        match &mut self.mode {
            PathInputMode::Normal => {
                if key_event.code == KeyCode::Enter {
                    self.mode = PathInputMode::Explorer;
                    true
                } else {
                    false
                }
            }
            PathInputMode::Explorer => match key_event.code {
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q' | 'Q') => {
                    self.mode = PathInputMode::Normal;
                    true
                }
                KeyCode::Char('a' | 'A') => {
                    self.mode = PathInputMode::Creating(SingleLineInput::new(None));
                    true
                }
                _ => {
                    if self.input.handle(&Event::Key(key_event)).is_err() {
                        self.mode = PathInputMode::Error("Something went wrong!".into())
                    }
                    true
                }
            },
            PathInputMode::Creating(textarea) => match key_event.code {
                KeyCode::Enter => {
                    if self.create_dir().is_err() {
                        self.mode = PathInputMode::Error("Something went wrong!".into())
                    }
                    self.mode = PathInputMode::Explorer;
                    true
                }
                _ => {
                    textarea.input(key_event);
                    true
                }
            },
            _ => false,
        }
    }

    fn create_dir(&mut self) -> std::io::Result<()> {
        let mode = std::mem::replace(&mut self.mode, PathInputMode::Explorer);
        if let PathInputMode::Creating(textarea) = mode {
            if let Some(foldername) = textarea.result() {
                let path = self.input.cwd().join(foldername);
                fs::create_dir(&path)?;
                self.input.set_cwd(path)?;
            }
        }

        Ok(())
    }

    pub fn result(self) -> PathBuf {
        self.input.current().path.clone()
    }
    pub fn render(&mut self, label: &str, selected: bool, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(label)
            .border_type(BorderType::Rounded);

        if selected {
            block = block.style(THEME.popup.selected);
        }
        let text = match &self.mode {
            PathInputMode::Error(message) => {
                block = block
                    .title_bottom(Line::raw(format!("[{}]", message)).right_aligned())
                    .style(THEME.error);
                self.original.to_string_lossy()
            }
            _ => {
                block = block.title_bottom(Line::raw("[Enter: Select]").right_aligned());
                self.input.current().path.to_string_lossy()
            }
        };

        let text_area = block.inner(area);
        let line = Line::raw(text);
        line.render(text_area, buf);
        block.render(area, buf);
    }

    pub fn render_popup(&mut self, area: Rect, buf: &mut Buffer) {
        match &mut self.mode {
            PathInputMode::Explorer => {
                Clear.render(area, buf);
                self.input.widget().render_ref(area, buf);
            }
            PathInputMode::Creating(textarea) => {
                Clear.render(area, buf);
                self.input.widget().render_ref(area, buf);
                let centered_area = area.centered(Constraint::Max(80), Constraint::Max(3));
                Clear.render(centered_area, buf);
                textarea.render("Foldername", true, centered_area, buf);
            }
            _ => {}
        }
    }

    pub fn new(value: Option<PathBuf>) -> Self {
        let path = value.unwrap_or(PathBuf::from(
            env::var("HOME").unwrap_or(String::from("/home/")),
        ));
        let fe = FileExplorerBuilder::default()
            .working_file(path.clone())
            .theme(explorer_theme())
            .build();
        if let Ok(explorer) = fe {
            PathInput {
                input: explorer,
                original: path,
                mode: PathInputMode::Normal,
            }
        } else {
            PathInput {
                mode: PathInputMode::Error("The provided Path seems to be invalid!".into()),
                original: path,
                input: FileExplorerBuilder::default().build().unwrap(),
            }
        }
    }
}

#[derive(Debug)]
pub struct SingleLineInput {
    pub input: TextArea<'static>,
}

impl SingleLineInput {
    pub fn new(value: Option<String>) -> Self {
        let mut ta = if let Some(s) = value {
            TextArea::from(vec![s.replace("\n", "")])
        } else {
            TextArea::default()
        };
        ta.move_cursor(CursorMove::End);
        ta.set_cursor_line_style(Style::default());
        SingleLineInput { input: ta }
    }
    pub fn input(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Enter => false,
            _ => self.input.input(Input::from(key_event)),
        }
    }

    pub fn result(self) -> Option<String> {
        self.input.into_lines().first().and_then(|s| {
            let s = s.trim().to_string();
            if s.is_empty() { None } else { Some(s) }
        })
    }

    pub fn render(&mut self, label: &str, selected: bool, area: Rect, buf: &mut Buffer) {
        render_text_area(&mut self.input, label, selected, area, buf);
    }
}

#[derive(Debug)]
pub struct MultiLineInput {
    pub input: TextArea<'static>,
}

impl MultiLineInput {
    pub fn new(value: Option<String>) -> Self {
        let mut ta = if let Some(s) = value {
            TextArea::from(s.split("\n").map(|s| s.to_string()))
        } else {
            TextArea::default()
        };
        ta.set_wrap_mode(WrapMode::WordOrGlyph);
        ta.move_cursor(CursorMove::End);
        ta.set_cursor_line_style(Style::default());
        MultiLineInput { input: ta }
    }

    pub fn input(&mut self, key_event: KeyEvent) -> bool {
        self.input.input(Input::from(key_event))
    }

    pub fn result(self) -> Option<String> {
        Some(
            self.input
                .into_lines()
                .iter()
                .fold(String::new(), |start, line| format!("{start}{line}\n")),
        )
    }
    pub fn render(&mut self, label: &str, selected: bool, area: Rect, buf: &mut Buffer) {
        render_text_area(&mut self.input, label, selected, area, buf);
    }
}

fn render_text_area(
    input: &mut TextArea<'static>,
    label: &str,
    selected: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    let mut block = Block::bordered()
        .title(label.to_string())
        .border_type(BorderType::Rounded);
    if selected {
        input.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        block = block.style(THEME.popup.selected);
    } else {
        input.set_cursor_style(Style::default());
    }
    input.set_block(block);
    input.render(area, buf);
}

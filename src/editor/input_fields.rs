use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{Buffer, Rect, Style, Widget};
use ratatui::style::Modifier;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, WidgetRef};
use ratatui_textarea::{CursorMove, Input, Key, TextArea, WrapMode};

use crate::utils::{THEME, explorer_theme};
use ratatui_explorer::{FileExplorer, FileExplorerBuilder};
use std::env;
use std::path::PathBuf;
use std::string::ToString;

pub trait InputField {
    type Value;
    fn new(value: Option<Self::Value>) -> Self;
    fn input(&mut self, key_event: KeyEvent) -> bool;
    fn result(self) -> Option<Self::Value>;
    fn render(&mut self, label: &str, selected: bool, area: Rect, buf: &mut Buffer);
}

#[derive(Debug)]
pub struct SingleLineInput {
    pub input: TextArea<'static>,
}

impl InputField for SingleLineInput {
    type Value = String;

    fn new(value: Option<String>) -> Self {
        let mut ta = if let Some(s) = value {
            TextArea::from(vec![s.replace("\n", "")])
        } else {
            TextArea::default()
        };
        ta.move_cursor(CursorMove::End);
        ta.set_cursor_line_style(Style::default());
        SingleLineInput { input: ta }
    }

    fn input(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Enter => false,
            _ => self.input.input(Input::from(key_event)),
        }
    }

    fn result(self) -> Option<String> {
        self.input.into_lines().first().and_then(|s| {
            let s = s.trim().to_string();
            if s.is_empty() { None } else { Some(s) }
        })
    }
    fn render(&mut self, label: &str, selected: bool, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(label.to_string())
            .border_type(BorderType::Rounded);
        if selected {
            self.input
                .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            block = block.style(THEME.popup.selected);
        } else {
            self.input.set_cursor_style(Style::default());
        }
        self.input.set_block(block);
        self.input.render(area, buf);
    }
}
#[derive(Debug)]
pub struct PathInput {
    pub input: FileExplorer,
}

impl InputField for PathInput {
    type Value = PathBuf;

    fn new(value: Option<PathBuf>) -> Self {
        let path = value.unwrap_or(PathBuf::from(
            env::var("HOME").unwrap_or(String::from("/home/")),
        ));
        PathInput {
            input: FileExplorerBuilder::default()
                .working_file(path)
                // .working_dir(path)
                .theme(explorer_theme())
                .build()
                .expect("Input invalid path"),
        }
    }

    fn input(&mut self, key_event: KeyEvent) -> bool {
        self.input.handle(&Event::Key(key_event)).is_ok()
    }

    fn result(self) -> Option<PathBuf> {
        Some(self.input.current().path.clone())
    }
    fn render(&mut self, label: &str, selected: bool, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(label)
            .title_bottom(Line::raw("[Enter: Select]").right_aligned())
            .border_type(BorderType::Rounded);
        if selected {
            block = block.style(THEME.popup.selected);
        }
        let text_area = block.inner(area);
        let line = Line::raw(self.input.current().path.display().to_string());
        line.render(text_area, buf);
        block.render(area, buf);
    }
}

impl PathInput {
    pub fn render_popup(&self, area: Rect, buf: &mut Buffer) {
        self.input.widget().render_ref(area, buf);
    }
    pub fn result_infallinble(self) -> PathBuf {
        self.input.current().path.clone()
    }
    pub fn try_new(value: Option<PathBuf>) -> Result<Self, std::io::Error> {
        let path = value.unwrap_or(PathBuf::from(
            env::var("HOME").unwrap_or(String::from("/home/")),
        ));
        Ok(PathInput {
            input: FileExplorerBuilder::default()
                .working_file(path)
                .theme(explorer_theme())
                .build()?,
        })
    }
}

#[derive(Debug)]
pub struct MultiLineInput {
    pub input: TextArea<'static>,
}

impl InputField for MultiLineInput {
    type Value = String;
    fn new(value: Option<String>) -> Self {
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

    fn input(&mut self, key_event: KeyEvent) -> bool {
        self.input.input(Input::from(key_event))
    }

    fn result(self) -> Option<String> {
        Some(
            self.input
                .into_lines()
                .iter()
                .fold(String::new(), |start, line| format!("{start}{line}\n")),
        )
    }
    fn render(&mut self, label: &str, selected: bool, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(label.to_string())
            .border_type(BorderType::Rounded);
        if selected {
            self.input
                .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            block = block.style(THEME.popup.selected);
        } else {
            self.input.set_cursor_style(Style::default());
        }
        self.input.set_block(block);
        self.input.render(area, buf);
    }
}

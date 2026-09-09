use super::{EditorEvent, MultiLineInput, PathInput, SingleLineInput};
use crate::persistence::{Category, Project};
use crate::utils::{THEME, render_title};

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{Buffer, Constraint, Layout, Line, Rect, Widget};
use ratatui::widgets::{Block, BorderType, Clear, Padding};

use std::cell::OnceCell;
use std::string::ToString;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Fields {
    Name,
    Category,
    Path,
    Tags,
    Lang,
    Desc,
}

impl Fields {
    fn next(self) -> Fields {
        match self {
            Fields::Name => Fields::Category,
            Fields::Category => Fields::Path,
            Fields::Path => Fields::Tags,
            Fields::Tags => Fields::Lang,
            Fields::Lang => Fields::Desc,
            Fields::Desc => Fields::Name,
        }
    }
    fn prev(self) -> Fields {
        match self {
            Fields::Name => Fields::Desc,
            Fields::Category => Fields::Name,
            Fields::Path => Fields::Category,
            Fields::Tags => Fields::Path,
            Fields::Lang => Fields::Tags,
            Fields::Desc => Fields::Lang,
        }
    }
}

#[derive(Debug)]
pub struct ProjectEditor {
    name: SingleLineInput,
    category: SingleLineInput,
    path: PathInput,
    tags: SingleLineInput,
    lang: SingleLineInput,
    desc: MultiLineInput,
    selected: Fields,
}

impl ProjectEditor {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<EditorEvent> {
        if self.selected == Fields::Path && self.path.input(key_event) {
            return None;
        }

        match key_event.code {
            KeyCode::Esc => {
                return Some(EditorEvent::Cancel);
            }
            KeyCode::Char('s' | 'S') if key_event.modifiers == KeyModifiers::CONTROL => {
                return Some(EditorEvent::Save);
            }
            KeyCode::Tab => self.selected = self.selected.next(),
            KeyCode::BackTab => self.selected = self.selected.prev(),
            _ => {
                match self.selected {
                    Fields::Name => self.name.input(key_event),
                    Fields::Category => self.category.input(key_event),
                    Fields::Tags => self.tags.input(key_event),
                    Fields::Lang => self.lang.input(key_event),
                    Fields::Desc => self.desc.input(key_event),
                    Fields::Path => false,
                };
            }
        }
        None
    }

    pub fn into_project(self) -> (String, Project) {
        let tags = self
            .tags
            .result()
            .unwrap_or_default()
            .split(",")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        (
            self.category
                .result()
                .unwrap_or(String::from("Uncategorized")),
            Project {
                path: self.path.result(),
                name: self.name.result().unwrap_or(String::from("Unnamed")),
                tags,
                language: self.lang.result(),
                description: self.desc.result(),
                preview: OnceCell::new(),
            },
        )
    }
}

impl Widget for &mut ProjectEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let centered_area = area.centered(Constraint::Percentage(80), Constraint::Percentage(80));
        Clear.render(centered_area, buf);
        let usage = Line::from("[Tab: Next] [S-Tab: Previous] [Esc: Cancel] [C-s: Save]")
            .style(THEME.key_bindings)
            .centered();
        let popup_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1))
            .title(render_title("Edit"))
            .title_bottom(usage)
            .style(THEME.popup.root);
        let vertical: [Rect; 5] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .areas(popup_block.inner(centered_area));
        let [name_rect, cat_rect] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(vertical[0]);
        self.name
            .render("Name", self.selected == Fields::Name, name_rect, buf);
        self.category
            .render("Category", self.selected == Fields::Category, cat_rect, buf);
        self.path
            .render("Path", self.selected == Fields::Path, vertical[1], buf);
        self.tags
            .render("Tags", self.selected == Fields::Tags, vertical[2], buf);
        self.lang
            .render("Lang", self.selected == Fields::Lang, vertical[3], buf);
        self.desc.render(
            "Description",
            self.selected == Fields::Desc,
            vertical[4],
            buf,
        );
        popup_block.render(centered_area, buf);
        self.path.render_popup(centered_area, buf);
    }
}

impl Default for ProjectEditor {
    fn default() -> Self {
        ProjectEditor {
            selected: Fields::Name,
            name: SingleLineInput::new(None),
            category: SingleLineInput::new(None),
            path: PathInput::new(None),
            tags: SingleLineInput::new(None),
            lang: SingleLineInput::new(None),
            desc: MultiLineInput::new(None),
        }
    }
}

impl From<(&Category, &Project)> for ProjectEditor {
    fn from(proj: (&Category, &Project)) -> Self {
        let (category, project) = proj;
        let tagstr = project
            .tags
            .iter()
            .fold(String::new(), |res, tag| format!("{res}{tag}, "));
        ProjectEditor {
            selected: Fields::Name,
            name: SingleLineInput::new(Some(project.name.clone())),
            category: SingleLineInput::new(Some(category.name.to_string())),
            path: PathInput::new(Some(project.path.clone())),
            tags: SingleLineInput::new(Some(tagstr)),
            lang: SingleLineInput::new(project.language.clone()),
            desc: MultiLineInput::new(project.description.clone()),
        }
    }
}

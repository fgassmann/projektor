use color_eyre::eyre::eyre;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Styled, Stylize},
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListDirection, ListItem, ListState,
        StatefulWidget, Widget,
    },
};

use crate::config::{EditMode, Project, ProjectList};
use crate::editor::{self, EditorEvent};
use crate::event::AppEvent;
use crate::utils::{THEME, render_title};

impl Widget for &mut ProjectList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        let [left, right] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(3)]).areas(top);
        self.render_list(left, buf);
        self.render_preview(right, buf);
        self.render_usage(bottom, buf);

        if let EditMode::EditingProject(e) | EditMode::CreatingProject(e) = &mut self.mode {
            e.render(area, buf);
        }
    }
}

impl ProjectList {
    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
    ) -> color_eyre::Result<Option<AppEvent>> {
        match &mut self.mode {
            EditMode::ProjectView => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q' | 'Q') => return Ok(Some(AppEvent::Quit)),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    return Ok(Some(AppEvent::Quit));
                }
                KeyCode::Tab => return Ok(Some(AppEvent::NextTab)),
                KeyCode::BackTab => return Ok(Some(AppEvent::PrevTab)),
                KeyCode::Char('j' | 'J') | KeyCode::Down => self.next(),
                KeyCode::Char('k' | 'K') | KeyCode::Up => self.prev(),
                KeyCode::Char('e' | 'E') => self.edit_project()?,
                KeyCode::Char('n' | 'N') => self.new_project(),

                KeyCode::Char('d' | 'D') => self.del_selected(),
                KeyCode::Char('f' | 'F') => self.mode = EditMode::EditFilter,
                KeyCode::Enter => {}
                _ => {}
            },
            EditMode::EditingProject(editor) | EditMode::CreatingProject(editor) => {
                match editor.handle_key_event(key_event)? {
                    Some(EditorEvent::Save) => {
                        self.save_project();
                        return Ok(Some(AppEvent::Save));
                    }
                    Some(EditorEvent::Cancel) => {
                        self.mode = EditMode::ProjectView;
                    }
                    None => {}
                }
            }

            EditMode::EditFilter => match key_event.code {
                KeyCode::Esc => self.mode = EditMode::ProjectView,
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    return Ok(Some(AppEvent::Quit));
                }
                KeyCode::Tab => return Ok(Some(AppEvent::NextTab)),
                KeyCode::BackTab => return Ok(Some(AppEvent::PrevTab)),
                KeyCode::Down => self.next(),
                KeyCode::Up => self.prev(),
                KeyCode::Backspace => {
                    self.filter.pop();
                }
                KeyCode::Char(c) => {
                    self.filter.push(c);
                }
                KeyCode::Enter => {}
                _ => {}
            },
        }
        Ok(None)
    }

    fn render_list(&self, area: Rect, buf: &mut Buffer) {
        let mut items: Vec<ListItem> = Vec::new();

        for (category, projects) in self.filtered_categories() {
            items.push(ListItem::from(format!("▼ {}", category.name)).bold());
            for p in projects {
                if self.filter.is_empty() || p.name.contains(&self.filter) {
                    items.push(ListItem::from(format!("  {}", p.name)));
                }
            }
        }

        let title = if !self.filter.is_empty() || matches!(self.mode, EditMode::EditFilter) {
            format!("Projects/ {}", self.filter)
        } else {
            String::from("Projects")
        };

        let block = Block::bordered()
            .title(render_title(&title))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);
        let list = List::new(items)
            .direction(ListDirection::TopToBottom)
            .highlight_style(Color::Blue)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always);
        let mut mod_state = ListState::default().with_selected(self.state.selected());

        StatefulWidget::render(list, area, buf, &mut mod_state);
    }

    fn render_usage(&self, area: Rect, buf: &mut Buffer) {
        let line = Line::from(
            "[J/▲: Up] [K/▼: Down] [Enter: Select] [e: Edit] [n: New] [d: Delete] [f: Filter]",
        )
        .style(THEME.key_bindings)
        .centered();
        line.render(area, buf);
    }

    fn render_preview(&self, area: Rect, buf: &mut Buffer) {
        let project = self.get_from_rendered();
        if let Some(p) = project {
            p.1.render(area, buf);
        } else {
            // todo!("mkae something kinda like a landing page?")
        }
    }

    fn save_project(&mut self) {
        let mode = std::mem::replace(&mut self.mode, EditMode::ProjectView);
        match mode {
            EditMode::EditingProject(p) => {
                let (cat, proj) = p.into();
                self.update_selected(proj, cat);
            }
            EditMode::CreatingProject(p) => {
                let (cat, proj) = p.into();
                self.insert(proj, cat);
            }
            _ => {
                unreachable!(
                    "Error: trying to save while not creating or editing a project. This should not happen."
                )
            }
        }
    }

    fn edit_project(&mut self) -> color_eyre::Result<()> {
        if let Some(p) = self.get_from_rendered() {
            self.mode = EditMode::EditingProject(editor::ProjectEditor::from(p))
        } else {
            return Err(eyre!("Nothing Selected"));
        }
        Ok(())
    }
    fn new_project(&mut self) {
        self.mode = EditMode::CreatingProject(editor::ProjectEditor::default())
    }
}

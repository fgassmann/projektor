use color_eyre::eyre::eyre;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{
        Block, BorderType, Clear, HighlightSpacing, List, ListDirection, ListItem, StatefulWidget,
        Widget,
    },
};

use crate::config::Settings;
use crate::datamodel::{EditMode, Entry, ProjectListView};
use crate::editor::{self, EditorEvent};
use crate::event::AppEvent;
use crate::utils::{THEME, render_title};

impl Widget for &mut ProjectListView {
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

impl ProjectListView {
    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        settings: &Settings,
    ) -> color_eyre::Result<Option<AppEvent>> {
        match &mut self.mode {
            EditMode::ProjectView => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q' | 'Q') => return Ok(Some(AppEvent::Quit)),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    return Ok(Some(AppEvent::Quit));
                }
                KeyCode::Enter => return Ok(Some(AppEvent::QuitWithSelected)),
                KeyCode::Tab => return Ok(Some(AppEvent::NextTab)),
                KeyCode::BackTab => return Ok(Some(AppEvent::PrevTab)),
                KeyCode::Char('j' | 'J') | KeyCode::Down => self.next(),
                KeyCode::Char('k' | 'K') | KeyCode::Up => self.prev(),
                KeyCode::Char('e' | 'E') => self.edit_project()?,
                KeyCode::Char('n' | 'N') => self.new_project(settings),
                KeyCode::Char('d' | 'D') => {
                    self.del_selected();
                    return Ok(Some(AppEvent::Save));
                }
                KeyCode::Char('f' | 'F') => self.mode = EditMode::EditFilter,
                _ => {}
            },
            EditMode::EditingProject(editor) | EditMode::CreatingProject(editor) => {
                match editor.handle_key_event(key_event) {
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

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .entries()
            .iter()
            .map(|e| match e {
                Entry::Header(ci) => {
                    ListItem::from(format!("▼ {}", self.categories[*ci].name)).bold()
                }
                Entry::Project { category, index } => ListItem::from(format!(
                    "  {}",
                    self.categories[*category].projects[*index].name
                )),
            })
            .collect();

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

        StatefulWidget::render(list, area, buf, &mut self.state);
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
            let logo = Text::styled(
                r"
  ___          _        _           
 | _ \_ _ ___ (_)___ __| |_ ___ _ _ 
 |  _/ '_/ _ \| / -_) _|  _/ _ \ '_|
 |_| |_| \___// \___\__|\__\___/_|  
            |__/

 -----------------------------------
            ",
                THEME.title,
            );
            let tooltip = Paragraph::new(Line::from(vec![
                Span::raw("No project selected. Select a project with "),
                Span::styled("[J/▲/K/▼]", THEME.description),
                Span::raw(" or create a new project with "),
                Span::styled("[N]", THEME.description),
                Span::raw("."),
            ]))
            .alignment(ratatui::layout::HorizontalAlignment::Center)
            .wrap(Wrap::default());
            let block = Block::bordered().border_type(BorderType::Rounded);
            let centered_area = block.inner(area).centered(
                Constraint::Length(logo.width().try_into().unwrap()),
                Constraint::Fill(1),
            );
            let vertical: [Rect; 3] = Layout::vertical([
                Constraint::Length(logo.height().try_into().unwrap()),
                Constraint::Length(5),
                Constraint::Max(10),
            ])
            .flex(ratatui::layout::Flex::Center)
            .areas(centered_area);
            block.render(area, buf);
            Clear.render(centered_area, buf);
            logo.render(vertical[0], buf);
            tooltip.render(vertical[1], buf);
        }
    }

    fn save_project(&mut self) {
        let mode = std::mem::replace(&mut self.mode, EditMode::ProjectView);
        match mode {
            EditMode::EditingProject(p) => {
                let (cat, proj) = p.into_project();
                self.update_selected(proj, cat);
            }
            EditMode::CreatingProject(p) => {
                let (cat, proj) = p.into_project();
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
    fn new_project(&mut self, settings: &Settings) {
        self.mode = EditMode::CreatingProject(editor::ProjectEditor::new(settings))
    }
}

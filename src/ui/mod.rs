use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect, Spacing},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Clear, HighlightSpacing, List, ListDirection, ListItem, ListState,
        Paragraph, StatefulWidget, Tabs, Widget,
    },
};

use crate::app::{App, AppMode};
use crate::utils::THEME;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [nav, top, bottom] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);
        let [left, right] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(3)]).areas(top);
        self.config.projects.render(left, buf);
        self.render_nav_bar(nav, buf);
        self.render_preview(right, buf);
        self.render_usage(bottom, buf);

        if let AppMode::EditingProject(e) | AppMode::CreatingProject(e) = &mut self.mode {
            e.render(area, buf);
        }
        if let AppMode::Error(msg, _) = &self.mode {
            self.render_error(msg, bottom, buf);
        }
    }
}

impl App {
    fn render_nav_bar(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_type(BorderType::Rounded);
        let tabs =
            Tabs::new(["Projects", "Templates", "Settings"].map(|s| Span::styled(s, THEME.tabs)))
                .select(0)
                .highlight_style(THEME.tabs_selected)
                .block(block);
        tabs.render(area, buf);
    }

    fn render_usage(&self, area: Rect, buf: &mut Buffer) {
        let line =
            Line::from("[J/▲: Up] [K/▼: Down] [Enter: Select] [e: Edit] [n: New] [d: Delete]")
                .style(THEME.key_bindings)
                .centered();
        line.render(area, buf);
    }
    fn render_error(&self, err: &str, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let line = Line::from(format!("Error: {}  (Press any key to continue)", err))
            .style(THEME.error)
            .centered();
        line.render(area, buf);
    }

    fn render_preview(&self, area: Rect, buf: &mut Buffer) {
        let project = self.config.projects.get_from_rendered();
        if let Some(p) = project {
            p.1.render(area, buf);
        }
    }
}

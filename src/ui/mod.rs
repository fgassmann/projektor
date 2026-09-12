use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Tabs, Widget},
};

use crate::app::{App, Popup};
use crate::utils::THEME;
mod project;
mod projectview;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [nav, main] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        self.render_nav_bar(nav, buf);
        self.projects.render(main, buf);

        if let Some(msg) = &self.popups.last() {
            let [_, keybinds] =
                Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

            match msg {
                Popup::Error(e) => self.render_error(e, keybinds, buf),
            }
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

    fn render_error(&self, err: &str, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let line = Line::from(format!("Error: {}  (Press any key to continue)", err))
            .style(THEME.error)
            .centered();
        line.render(area, buf);
    }
}

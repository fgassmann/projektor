use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::text::Span;

pub mod markdown;
mod theme;
pub use theme::{THEME, explorer_theme};

const TITLE_LEFT: &str = "┐";
const TITLE_RIGHT: &str = "┌";

pub fn render_title(title: &str) -> Vec<Span<'_>> {
    vec![
        Span::from(TITLE_LEFT),
        Span::from(title).yellow().bold(),
        Span::from(TITLE_RIGHT),
    ]
}

pub fn render_tag(tag: &str) -> Vec<Span<'_>> {
    let hash = tag
        .bytes()
        .fold(0, |acc: u16, b| acc.wrapping_add(b as u16))
        % 5;
    let (bg, fg) = match hash {
        0 => (Color::Magenta, Color::White),
        1 => (Color::Green, Color::Black),
        2 => (Color::Yellow, Color::White),
        3 => (Color::Blue, Color::Black),
        _ => (Color::White, Color::Black),
    };
    let s = Style::new().fg(bg);
    vec![
        Span::styled("", s),
        Span::styled(tag.to_string(), Style::new().bg(bg).fg(fg)),
        Span::styled(" ", s),
    ]
}

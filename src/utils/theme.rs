use std::rc::Rc;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui_explorer;

const BROWN_NEUTRAL: Color = Color::Rgb(62, 59, 57); //BG
const GRAY_BRIGHT: Color = Color::Rgb(222, 222, 222); //FG

const BROWN_LIGHT: Color = Color::Rgb(75, 72, 69); //POPUP_BG

const GRAY_DARK: Color = Color::Rgb(154, 154, 154); // MUTED
const YELLOW_NEUTRAL: Color = Color::Rgb(253, 175, 31); //Headings & Highlight
const BLUE: Color = Color::Rgb(74, 170, 214);
#[allow(dead_code)]
const ORANGE: Color = Color::Rgb(255, 149, 0);
const RED: Color = Color::Rgb(255, 34, 0);

pub struct Theme {
    pub root: Style,
    pub title: Style,
    pub tabs: Style,
    pub tabs_selected: Style,
    pub borders: Style,
    pub description: Style,
    pub muted: Style,
    pub error: Style,
    pub key_bindings: Style,
    pub popup: Popup,
}

pub struct Popup {
    pub root: Style,
    pub selected: Style,
    pub accent: Style,
}

pub const THEME: Theme = Theme {
    root: Style::new().bg(BROWN_NEUTRAL).fg(GRAY_BRIGHT),
    title: Style::new().fg(YELLOW_NEUTRAL).add_modifier(Modifier::BOLD),
    tabs: Style::new().fg(BLUE),
    tabs_selected: Style::new().fg(GRAY_BRIGHT).add_modifier(Modifier::BOLD),
    borders: Style::new().fg(GRAY_BRIGHT),
    description: Style::new().fg(GRAY_DARK),
    key_bindings: Style::new().fg(GRAY_DARK),
    muted: Style::new().fg(GRAY_DARK),
    error: Style::new().fg(RED).bold(),
    popup: Popup {
        root: Style::new().bg(BROWN_LIGHT),
        selected: Style::new().fg(YELLOW_NEUTRAL),
        accent: Style::new().fg(BLUE),
    },
};

pub fn explorer_theme() -> ratatui_explorer::Theme {
    ratatui_explorer::Theme::new()
        .with_block(Block::bordered())
        .add_default_title()
        .with_title_bottom(|_| Line::raw("[Enter/Esc: Select]").right_aligned())
        .with_style(THEME.popup.root)
        .with_highlight_symbol("> ")
        .with_dir_style(THEME.popup.accent)
        .with_highlight_dir_style(THEME.popup.selected)
        .with_highlight_item_style(THEME.popup.selected)
}

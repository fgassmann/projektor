use ratatui::style::Style;
use tui_markdown::StyleSheet;

#[derive(Debug, Clone)]
pub struct PreviewStyle;

impl StyleSheet for PreviewStyle {
    fn heading(&self, level: u8) -> Style {
        match level {
            1 => Style::new().yellow().bold(),
            _ => Style::new().magenta().bold(),
        }
    }

    fn code(&self) -> Style {
        Style::new().green()
    }

    fn link(&self) -> Style {
        Style::new().blue().underlined()
    }

    fn blockquote(&self) -> Style {
        Style::new().magenta().italic()
    }

    fn heading_meta(&self) -> Style {
        Style::new().green()
    }

    fn metadata_block(&self) -> Style {
        Style::new().light_yellow()
    }

    fn code_block_fence(&self) -> &str {
        ""
    }
}

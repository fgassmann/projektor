use crate::utils::markdown::PreviewStyle;
use crate::utils::{render_tag, render_title};

use ratatui::prelude::{Alignment, Buffer, Constraint, Layout, Rect, Style, Widget};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use std::fs;
use std::path::PathBuf;

use crate::persistence::Project;

impl Widget for &Project {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, bottom] =
            Layout::vertical([Constraint::Fill(2), Constraint::Fill(1)]).areas(area);
        //region README
        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .title_top(render_title("README"))
            .border_type(BorderType::Rounded);

        let text = self.preview.get_or_init(|| {
            fs::read_to_string(self.path.join(PathBuf::from("README.md")))
                .ok()
                .unwrap_or(String::from("No README in Project.\n"))
        });
        // todo!("Make this better so we don't reparse the file every frame...");
        let md_opts = tui_markdown::Options::new(PreviewStyle);
        let paragraph = &Paragraph::new(tui_markdown::from_str_with_options(text, &md_opts))
            // let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(block)
            .left_aligned();

        paragraph.render(top, buf);
        //endregion README

        //region INFOS
        let mut info_lines = vec![
            // Line::from(Span::raw(self.name.clone()).bold()),
            Line::from(vec![
                "Path: ".magenta(),
                self.path.to_string_lossy().italic(),
            ]),
            Line::from(vec![
                "Language: ".magenta(),
                self.language.clone().unwrap_or("Unknown".into()).into(),
            ]),
        ];
        if !self.tags.is_empty() {
            let rendered_tags: Vec<Span<'_>> =
                self.tags.iter().flat_map(|t| render_tag(t)).collect();
            info_lines.push(Line::from_iter(
                std::iter::once(Span::from("Tags: ").magenta()).chain(rendered_tags),
            ));
        }
        let description = Paragraph::new(self.description.clone().unwrap_or_default())
            .wrap(Wrap { trim: true })
            .style(Style::new().dark_gray());

        let block = Block::bordered()
            .title_alignment(Alignment::Left)
            .title_top(render_title(self.name.as_str())) //"Info"))
            .border_type(BorderType::Rounded);
        let block_area = block.inner(bottom);
        let [info_area, description_area] = Layout::vertical([
            Constraint::Length(info_lines.len().try_into().unwrap_or(5)),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .areas(block_area);

        description.render(description_area, buf);
        block.render(bottom, buf);
        let info = Paragraph::new(Text::from(info_lines));
        info.render(info_area, buf);
        //endregion INFOS
    }
}

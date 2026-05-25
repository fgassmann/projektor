use crate::utils::markdown::PreviewStyle;
use crate::utils::{render_tag, render_title};

use crate::editor;
use ratatui::prelude::{Alignment, Buffer, Color, Constraint, Layout, Rect, Style, Widget};
use ratatui::style::{Styled, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, BorderType, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph,
    StatefulWidget, Wrap,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Debug)]
pub enum EditMode {
    ProjectView,
    EditingProject(editor::ProjectEditor),
    CreatingProject(editor::ProjectEditor),
}

impl Default for EditMode {
    fn default() -> Self {
        Self::ProjectView
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct ProjectList {
    pub categories: Vec<Category>,
    #[serde(skip)]
    pub state: ListState,
    #[serde(skip)]
    pub mode: EditMode,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Category {
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    // pub default: Default,
    pub projects: ProjectList,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub description: Option<String>,
}

impl Config {
    pub fn save(&self) {
        let contents = toml::to_string_pretty(self).expect("Error: Unable to save modified config");
        fs::write("config_out.toml", contents).expect("Error: Unable to save modified config");
    }
    pub fn load(path: PathBuf) -> Self {
        todo!("TODO")
    }
}

impl ProjectList {
    pub fn get_project(&self, mut index: usize) -> Option<(&str, &Project)> {
        for category in &self.categories {
            if index < category.projects.len() {
                return Some((&category.name, &category.projects[index]));
            }
            index -= category.projects.len();
        }
        None
    }
    fn headermask(&self) -> Vec<bool> {
        let mut items: Vec<bool> = Vec::new();
        for category in self.categories.iter() {
            items.push(true);
            for _ in &category.projects {
                items.push(false);
            }
        }
        items
    }
    pub fn next(&mut self) {
        let mask = self.headermask();
        self.state.select_next();
        let index = self
            .state
            .selected()
            .expect("Error: Couldn't select next Project. This should never happen.");
        if let Some(m) = mask.get(index) {
            if *m {
                self.state.select_next();
            }
        } else {
            self.state.select_previous();
        }
    }
    pub fn prev(&mut self) {
        let mask = self.headermask();
        self.state.select_previous();
        let index = self
            .state
            .selected()
            .expect("Error: Couldn't select previous Project. This should never happen.");
        if let Some(m) = mask.get(index)
            && index > 0
        {
            if *m {
                self.state.select_previous();
            }
        } else {
            self.state.select_next();
        }
    }
    pub fn get_from_rendered(&self) -> Option<(&str, &Project)> {
        if let Some(mut render_idx) = self.state.selected() {
            for category in &self.categories {
                if render_idx == 0 {
                    return None; // header selected
                }
                render_idx -= 1;
                if render_idx < category.projects.len() {
                    return Some((&category.name, &category.projects[render_idx]));
                }
                render_idx -= category.projects.len();
            }
        }
        None
    }

    pub fn insert(&mut self, project: Project, category: String) {
        if let Some(cat) = self.categories.iter_mut().find(|c| c.name == category) {
            cat.projects.push(project);
        } else {
            self.categories.push(Category {
                name: category,
                projects: vec![project],
            })
        }
    }

    pub fn update_selected(&mut self, project: Project, category: String) {
        self.del_selected();
        self.insert(project, category);
    }
    pub fn del_selected(&mut self) {
        if let Some(mut render_idx) = self.state.selected() {
            for category in &mut self.categories {
                if render_idx == 0 {
                    return; // header selected
                }
                render_idx -= 1;
                if render_idx < category.projects.len() {
                    category.projects.remove(render_idx);
                    return;
                }
                render_idx -= category.projects.len();
            }
        }
    }
}

impl Widget for &Project {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, bottom] =
            Layout::vertical([Constraint::Fill(2), Constraint::Fill(1)]).areas(area);
        //region README
        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .title_top(render_title("README"))
            .border_type(BorderType::Rounded);

        let text = fs::read_to_string(self.path.join(PathBuf::from("README.md")))
            .ok()
            .unwrap_or(String::from("No README in Project.\n"));
        let md_opts = tui_markdown::Options::new(PreviewStyle);
        let paragraph = Paragraph::new(tui_markdown::from_str_with_options(&text, &md_opts))
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

pub fn get_test_config() -> Config {
    Config {
        projects: ProjectList {
            state: ListState::default(),
            mode: EditMode::ProjectView,
            categories: vec![
                Category {
                    name: String::from("Uncategorized"),

                    projects: vec![
                        Project {
                            path: PathBuf::from("/home/fgassmann/Projects/Rust/projektor"),
                            name: String::from("Projektor"),
                            tags: vec!["Tui".into()],
                            language: Some("Rust".into()),
                            description: Some(String::from(
                                "Utility to keep track of Projects from the commandline.\nTrust me It's very cool. I just don't know what eslse to say about it.\n Bla",
                            )),
                        },
                        Project {
                            path: PathBuf::from(
                                "/home/fgassmann/Projects/Rust/gdm-wallpaper-compositor",
                            ),
                            name: String::from("GDM-wallpaper-compositor"),
                            tags: vec!["GDM".into(), "Ricing".into()],
                            language: Some("Rust".into()),
                            description: None,
                        },
                        Project {
                            path: PathBuf::from("/home/fgassmann/Projects/Upstream/ratatui"),
                            name: String::from("Ratatui"),
                            tags: vec!["Tui".into()],
                            language: Some("Rust".into()),
                            description: None,
                        },
                        Project {
                            path: PathBuf::from("/home/fgassmann/Projects/Upstream/linux-retroism"),
                            name: String::from("Linux Retroism Rice"),
                            tags: vec!["Ricing".into(), "Wayland".into()],
                            language: None,
                            description: None,
                        },
                    ],
                },
                Category {
                    name: String::from("C"),
                    projects: vec![Project {
                        path: PathBuf::from("/home/fgassmann/Projects/C/connect4"),
                        name: String::from("Connect 4 Raylib"),
                        tags: vec![],
                        language: Some("C".into()),
                        description: Some(String::from(
                            "Trying out Raylib by implementing a simple Connect4 game. Has a strong solver the player can play against.",
                        )),
                    }],
                },
                Category {
                    name: String::from("Empty"),
                    projects: vec![],
                },
                Category {
                    name: String::from("Broken"),
                    projects: vec![Project {
                        path: PathBuf::from("/homes/fgassmann/Projects/C/connect4"),
                        name: String::from("Connect 4 Raylib Broken"),
                        tags: vec![],
                        language: Some("C".into()),
                        description: Some(String::from(
                            "Trying out Raylib by implementing a simple Connect4 game. Has a strong solver the player can play against.",
                        )),
                    }],
                },
            ],
        },
    }
}

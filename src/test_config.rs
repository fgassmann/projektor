use std::{cell::OnceCell, path::PathBuf};

use ratatui::widgets::ListState;

use crate::datamodel::{EditMode, ProjectListView};
use crate::persistence::*;
pub fn get_test_config() -> ProjectListView {
    ProjectListView {
        state: ListState::default(),
        mode: EditMode::ProjectView,
        filter: String::new(),
        projects: ProjectList {
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
                            preview: OnceCell::new(),
                        },
                        Project {
                            path: PathBuf::from(
                                "/home/fgassmann/Projects/Rust/gdm-wallpaper-compositor",
                            ),
                            name: String::from("GDM-wallpaper-compositor"),
                            tags: vec!["GDM".into(), "Ricing".into()],
                            language: Some("Rust".into()),
                            description: None,
                            preview: OnceCell::new(),
                        },
                        Project {
                            path: PathBuf::from("/home/fgassmann/Projects/Upstream/ratatui"),
                            name: String::from("Ratatui"),
                            tags: vec!["Tui".into()],
                            language: Some("Rust".into()),
                            description: None,
                            preview: OnceCell::new(),
                        },
                        Project {
                            path: PathBuf::from("/home/fgassmann/Projects/Upstream/linux-retroism"),
                            name: String::from("Linux Retroism Rice"),
                            tags: vec!["Ricing".into(), "Wayland".into()],
                            language: None,
                            description: None,
                            preview: OnceCell::new(),
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
                        preview: OnceCell::new(),
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
                        preview: OnceCell::new(),
                    }],
                },
            ],
        },
    }
}

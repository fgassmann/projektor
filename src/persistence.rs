use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::cell::OnceCell;
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Serialize, Debug)]
// #[serde(transparent)]
pub struct ProjectList {
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Category {
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub description: Option<String>,
    #[serde(skip)]
    pub preview: OnceCell<String>,
}

impl ProjectList {
    pub fn save(&self) {
        let contents = toml::to_string_pretty(self).unwrap(); //.expect("Error: Unable to save modified config");
        fs::write("config_out.toml", contents).expect("Error: Unable to save modified config");
    }
    pub fn load(path: PathBuf) -> color_eyre::Result<Self> {
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(|_| eyre!("error parsing config"))
    }
}

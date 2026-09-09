use color_eyre::eyre::{Context, eyre};
use serde::{Deserialize, Serialize};
use std::cell::OnceCell;
use std::env::home_dir;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
// #[serde(transparent)]
pub struct ProjectList {
    pub categories: Vec<Category>,
    #[serde(skip)]
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
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
    pub fn save(&self) -> color_eyre::Result<()> {
        let contents = toml::to_string_pretty(self)
            .expect("Error converting data to toml. This should not happen!");
        if let Some(p) = self.path.parent()
            && !p.exists()
        {
            fs::create_dir_all(p).wrap_err(format!(
                "Error saving config {} (missing parent directories)",
                self.path.to_string_lossy()
            ))?;
        };
        fs::write(&self.path, contents).wrap_err(format!(
            "Error saving config {}",
            self.path.to_string_lossy()
        ))?;
        Ok(())
    }

    pub fn load(path: &Option<PathBuf>) -> color_eyre::Result<Self> {
        let path = if let Some(p) = path {
            p.clone()
        } else {
            home_dir()
                .ok_or(eyre!(
                    "Default config not found and no config path provided."
                ))?
                .join(".config")
                .join("projektor")
                .join("config.toml")
        };
        let conf = fs::read_to_string(&path);

        match conf {
            Ok(content) => {
                let mut data: Self =
                    toml::from_str(&content).wrap_err("Error parsing configuration file")?;
                data.path = path;
                Ok(data)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(ProjectList {
                categories: Vec::new(),
                path,
            }),
            Err(_) => Err(eyre!("Error loading configuration file.")),
        }
    }
}

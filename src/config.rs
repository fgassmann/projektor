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
pub struct Config {
    pub categories: Vec<Category>,
    pub settings: Settings,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    pub default_folder: Option<PathBuf>,
    #[serde(skip)]
    pub config_path: PathBuf,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Category {
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
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

impl Config {
    pub fn save(&self) -> color_eyre::Result<()> {
        let contents = toml::to_string_pretty(self)
            .expect("Error converting data to toml. This should not happen!");
        if let Some(p) = self.settings.config_path.parent()
            && !p.exists()
        {
            // I'm not sure this is something that realistically happens
            fs::create_dir_all(p).wrap_err(format!(
                "Error saving config {} (missing parent directories)",
                p.to_string_lossy()
            ))?;
        };
        fs::write(&self.settings.config_path, contents).wrap_err(format!(
            "Error saving config {}",
            self.settings.config_path.to_string_lossy()
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

        match fs::read_to_string(&path) {
            Ok(content) => {
                let mut data: Self =
                    toml::from_str(&content).wrap_err("Error parsing configuration file")?;
                data.settings.config_path = path;
                Ok(data)
            }
            // If no config exists create a new one
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Config {
                categories: Vec::new(),
                settings: Settings {
                    default_folder: None,
                    config_path: path,
                },
            }),
            Err(_) => Err(eyre!("Error loading configuration file.")),
        }
    }
}

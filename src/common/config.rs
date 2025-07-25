use dirs::config_dir;
use serde::{Deserialize, Serialize};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use toml;

use crate::error::Error;
use crate::error::{Error::CustomError, Result};

pub struct ConfigParser {
    pub config: AppConfig,
    pub config_file: String,
}

impl ConfigParser {
    pub async fn parse() -> Result<Self> {
        let mut config_file = config_dir().expect("Something wrong with config path");
        config_file.push(std::env!("CARGO_PKG_NAME"));

        // Create directory structure if it doesn't exist
        if !fs::try_exists(&config_file).await? {
            fs::create_dir_all(&config_file).await?;
        }

        config_file.push("config.toml");
        if !fs::try_exists(&config_file).await? {
            let config_file_default_content = r#" clocks = [
    "clockify",
]
time_zone = 5.5
editor = "nvim"

[[clockify_projects]]
code = ""
key = ""
id = ""
        "#;
            // Create the file and write content
            let mut file = fs::File::create(&config_file).await?;
            file.write_all(config_file_default_content.as_bytes())
                .await?;
        }
        let mut content = String::new();
        let mut file = File::open(&config_file).await?;
        file.read_to_string(&mut content)
            .await
            .map_err(|e| CustomError(e.to_string()))?;
        let config: AppConfig = toml::from_str(&content).map_err(|e| CustomError(e.to_string()))?;
        Ok(Self {
            config,
            config_file: config_file.to_string_lossy().to_string(),
        })
    }

    pub fn set_project(
        &mut self,
        _type: &str,
        key: &String,
        code: String,
        id: String,
    ) -> Result<&mut Self> {
        let project = Project {
            id,
            code,
            key: key.to_owned(),
        };
        self.config
            .clockify_projects
            .get_or_insert_with(Vec::new)
            .push(project);

        Ok(self)
    }

    pub async fn update_config(&self) -> Result<()> {
        let config_string =
            toml::to_string_pretty(&self.config).map_err(|e| Error::CustomError(e.to_string()))?;

        fs::write(self.config_file.clone(), config_string).await?;
        Ok(())
    }

    pub fn get_clocks(&self) -> Result<Vec<String>> {
        Ok(self.config.clocks.clone())
    }

    pub fn get_projects(&mut self) -> Result<Vec<Project>> {
        Ok(self
            .config
            .clockify_projects
            .get_or_insert_with(Vec::new)
            .clone())
    }

    pub fn get_editor(self) -> Result<String> {
        Ok(self.config.editor.unwrap_or("".to_string()))
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct AppConfig {
    pub clocks: Vec<String>,
    pub editor: Option<String>,
    pub clockify_projects: Option<Vec<Project>>,
    pub jira_projects: Option<Vec<Project>>,
    pub time_zone: Option<f32>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            clocks: vec!["jira".to_string()],
            editor: None,
            clockify_projects: Option::None,
            jira_projects: Option::None,
            time_zone: Some(0.0),
        }
    }
}

#[derive(Default, Deserialize, Debug, Serialize, Clone)]
pub struct Project {
    pub code: String,
    pub key: String,
    pub id: String,
}

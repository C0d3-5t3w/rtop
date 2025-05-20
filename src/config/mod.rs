use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub update_interval: u64,
    pub theme: String,
    pub layout: LayoutConfig,
    pub sort_by: String,
    pub filters: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_network: bool,
    pub show_disk: bool,
    pub show_process_details: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_interval: 1000,
            theme: "default".to_string(),
            layout: LayoutConfig {
                show_cpu: true,
                show_memory: true,
                show_network: true,
                show_disk: true,
                show_process_details: true,
            },
            sort_by: "cpu".to_string(),
            filters: vec![],
        }
    }
}

impl Config {
    pub fn load(path: Option<&str>) -> Result<Self> {
        if let Some(config_path) = path {
            if Path::new(config_path).exists() {
                let content = fs::read_to_string(config_path)?;
                return Ok(toml::from_str(&content)?);
            }
        }
        
        // Look for config in default locations
        let home_config = dirs::config_dir()
            .map(|p| p.join("rtop/config.toml"))  // Fixed path - was looking in "../../pkg/config.toml"
            .filter(|p| p.exists());
            
        if let Some(path) = home_config {
            let content = fs::read_to_string(path)?;
            return Ok(toml::from_str(&content)?);
        }
        
        Ok(Config::default())
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

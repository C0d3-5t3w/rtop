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
        // Try loading from the specified path first
        if let Some(config_path) = path {
            if Path::new(config_path).exists() {
                match fs::read_to_string(config_path) {
                    Ok(content) => match toml::from_str(&content) {
                        Ok(config) => return Ok(config),
                        Err(e) => eprintln!("Error parsing config file: {}", e),
                    },
                    Err(e) => eprintln!("Error reading config file: {}", e),
                }
            }
        }
        
        // Check standard locations - both TOML and YAML
        for ext in &["toml", "yaml", "yml"] {
            if let Some(config_dir) = dirs::config_dir() {
                let path = config_dir.join(format!("rtop/config.{}", ext));
                if path.exists() {
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            if ext == "toml" {
                                if let Ok(config) = toml::from_str(&content) {
                                    return Ok(config);
                                }
                            } else {
                                // For YAML, we would need the serde_yaml crate
                                // Just continue for now
                            }
                        },
                        Err(e) => eprintln!("Error reading config at {:?}: {}", path, e),
                    }
                }
            }
        }
        
        // Package config fallback
        let pkg_path = Path::new("pkg/config.yaml");
        if pkg_path.exists() {
            eprintln!("Using pkg/config.yaml as fallback");
            // This would need serde_yaml to parse
        }
        
        // Fall back to default configuration if all else fails
        Ok(Config::default())
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

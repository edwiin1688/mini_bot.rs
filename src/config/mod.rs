use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

const ENV_API_KEY: &str = "MINIBOT_API_KEY";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub default_provider: String,
    pub default_model: String,
    #[serde(default)]
    pub api_key: String,
    pub gateway: GatewayConfig,
    pub agent: AgentConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub max_tool_iterations: usize,
    pub max_history_messages: usize,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub workspace_only: bool,
    pub allowed_roots: Vec<String>,
    pub allowed_commands: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            default_provider: "minimax".to_string(),
            default_model: "minimax-coding-plan/MiniMax-M2.5".to_string(),
            api_key: String::new(),
            gateway: GatewayConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            agent: AgentConfig {
                max_tool_iterations: 100,
                max_history_messages: 50,
                temperature: 0.7,
            },
            security: SecurityConfig {
                workspace_only: true,
                allowed_roots: vec![],
                allowed_commands: vec![],
            },
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;

        if let Ok(env_key) = env::var(ENV_API_KEY) {
            if !env_key.is_empty() {
                config.api_key = env_key;
            }
        }

        Ok(config)
    }

    pub fn api_key_from_env() -> Option<String> {
        env::var(ENV_API_KEY).ok().filter(|k| !k.is_empty())
    }

    #[allow(dead_code)]
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn config_dir() -> PathBuf {
        directories::ProjectDirs::from("com", "minibot", "mini_bot_rs")
            .map(|dirs| dirs.config_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    }

    pub fn default_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, "1.0");
        assert_eq!(config.default_provider, "minimax");
        assert_eq!(config.gateway.port, 3000);
        assert_eq!(config.agent.temperature, 0.7);
    }

    #[test]
    fn test_config_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("config.toml");

        let config = Config::default();
        config.save(&path).unwrap();

        let loaded = Config::load(&path).unwrap();
        assert_eq!(loaded.version, config.version);
        assert_eq!(loaded.default_provider, config.default_provider);
    }

    #[test]
    fn test_config_dir() {
        let dir = Config::config_dir();
        assert!(dir.to_string_lossy().len() > 0);
    }
}

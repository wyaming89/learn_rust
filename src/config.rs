use anyhow::{anyhow, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: String,
    pub base_url: String,
    pub is_sandbox: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("OKX_API_KEY")
            .map_err(|_| anyhow!("缺少环境变量 OKX_API_KEY"))?;
        
        let secret_key = env::var("OKX_SECRET_KEY")
            .map_err(|_| anyhow!("缺少环境变量 OKX_SECRET_KEY"))?;
        
        let passphrase = env::var("OKX_PASSPHRASE")
            .map_err(|_| anyhow!("缺少环境变量 OKX_PASSPHRASE"))?;
        
        let is_sandbox = env::var("OKX_SANDBOX")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);
        
        let base_url = "https://www.okx.com".to_string();

        Ok(Config {
            api_key,
            secret_key,
            passphrase,
            base_url,
            is_sandbox,
        })
    }
} 
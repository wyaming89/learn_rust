use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

use crate::{
    config::Config,
    rate_limiter::RateLimiter,
    types::{ApiResponse, PositionHistory},
};

#[derive(Debug, Clone, Serialize)]
pub struct PositionsHistoryParams {
    pub inst_type: Option<String>,
    pub inst_id: Option<String>,
    pub mgn_mode: Option<String>,
    pub close_type: Option<String>,
    pub pos_id: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub limit: Option<String>,
}

impl PositionsHistoryParams {
    pub fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        if let Some(ref inst_type) = self.inst_type {
            params.insert("instType".to_string(), inst_type.clone());
        }
        if let Some(ref inst_id) = self.inst_id {
            params.insert("instId".to_string(), inst_id.clone());
        }
        if let Some(ref mgn_mode) = self.mgn_mode {
            params.insert("mgnMode".to_string(), mgn_mode.clone());
        }
        if let Some(ref close_type) = self.close_type {
            params.insert("type".to_string(), close_type.clone());
        }
        if let Some(ref pos_id) = self.pos_id {
            params.insert("posId".to_string(), pos_id.clone());
        }
        if let Some(ref before) = self.before {
            params.insert("before".to_string(), before.clone());
        }
        if let Some(ref after) = self.after {
            params.insert("after".to_string(), after.clone());
        }
        if let Some(ref limit) = self.limit {
            params.insert("limit".to_string(), limit.clone());
        }
        
        params
    }
}

pub struct PositionsHistoryClient {
    client: Client,
    config: Config,
    rate_limiter: RateLimiter,
}

impl PositionsHistoryClient {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config,
            rate_limiter: RateLimiter::default(),
        }
    }

    pub async fn get_positions_history(&self, params: &PositionsHistoryParams) -> Result<ApiResponse<PositionHistory>> {
        // 检查限速
        self.rate_limiter.wait_if_needed(&self.config.api_key)?;

        // 构建URL
        let url = format!("{}/api/v5/account/positions-history", self.config.base_url);
        
        // 构建查询参数
        let query_params = params.to_query_params();
        
        // 发送请求
        let timestamp = self.get_timestamp().await?;
        let signature = self.generate_signature(&query_params, &timestamp)?;
        
        let response = self.client
            .get(&url)
            .query(&query_params)
            .header("OK-ACCESS-KEY", &self.config.api_key)
            .header("OK-ACCESS-SIGN", signature)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.config.passphrase)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "无法读取错误信息".to_string());
            return Err(anyhow!("HTTP错误: {} - {}", status, error_text));
        }

        // 解析JSON响应
        let api_response: ApiResponse<PositionHistory> = response.json().await?;
        
        if api_response.code != "0" {
            return Err(anyhow!("API错误: {} - {}", api_response.code, api_response.msg));
        }

        Ok(api_response)
    }

    fn generate_signature(&self, params: &HashMap<String, String>, timestamp: &str) -> Result<String> {
        let method = "GET";
        let request_path = "/api/v5/account/positions-history";
        
        // 构建签名字符串
        let mut sign_string = timestamp.to_string() + method + request_path;
        
        // 添加查询参数
        if !params.is_empty() {
            let mut sorted_params: Vec<_> = params.iter().collect();
            sorted_params.sort_by_key(|&(k, _)| k);
            
            let query_string = sorted_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            
            sign_string += "?";
            sign_string += &query_string;
        }
        
        // 使用HMAC-SHA256生成签名
        let mut mac = Hmac::<Sha256>::new_from_slice(self.config.secret_key.as_bytes())
            .map_err(|e| anyhow!("HMAC初始化失败: {}", e))?;
        
        mac.update(sign_string.as_bytes());
        let result = mac.finalize();
        

        
        Ok(base64::encode(result.into_bytes()))
    }

    async fn get_timestamp(&self) -> Result<String> {
        // 使用ISO 8601格式的时间戳，如：2020-12-08T09:08:57.715Z
        use chrono::Utc;
        let utc_now = Utc::now();
        // 格式化为ISO 8601格式，包含毫秒
        let timestamp = utc_now.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
        Ok(timestamp)
    }
}

pub async fn get_positions_history(
    config: &Config,
    params: &PositionsHistoryParams,
) -> Result<ApiResponse<PositionHistory>> {
    let client = PositionsHistoryClient::new(config.clone());
    client.get_positions_history(params).await
} 
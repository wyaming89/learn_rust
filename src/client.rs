use crate::{config::Config, rate_limiter::RateLimiter};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use base64;

/// 通用OKX API客户端
pub struct OkxClient {
    client: Client,
    config: Config,
    rate_limiter: RateLimiter,
}

impl OkxClient {
    /// 创建新的API客户端
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

    /// 执行GET请求
    pub async fn get<T>(&self, endpoint: &str, params: Option<&HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request::<T>("GET", endpoint, params, None::<&()>).await
    }

    /// 执行POST请求
    pub async fn post<T>(&self, endpoint: &str, body: Option<&impl Serialize>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request::<T>("POST", endpoint, None, body).await
    }

    /// 通用请求方法
    async fn request<T>(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        body: Option<&impl Serialize>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // 等待限速
        self.rate_limiter.wait_if_needed(&self.config.api_key)?;

        // 构建URL
        let url = format!("{}{}", self.config.base_url, endpoint);

        // 构建请求
        let mut request = match method {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            _ => return Err(anyhow!("不支持的HTTP方法: {}", method)),
        };

        // 添加查询参数
        if let Some(params) = params {
            for (key, value) in params {
                request = request.query(&[(key, value)]);
            }
        }

        // 添加请求体
        if let Some(body) = body {
            request = request.json(body);
        }

        // 生成认证头
        let timestamp = self.get_timestamp().await?;
        let signature = self.generate_signature(method, endpoint, params, &timestamp)?;

        // 添加请求头
        request = request
            .header("OK-ACCESS-KEY", &self.config.api_key)
            .header("OK-ACCESS-SIGN", signature)
            .header("OK-ACCESS-TIMESTAMP", &timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.config.passphrase)
            .header("Content-Type", "application/json");

        if self.config.is_sandbox {
            request = request.header("x-simulated-trading", "1");
        }

        // 发送请求
        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "无法读取错误信息".to_string());
            return Err(anyhow!("HTTP错误: {} - {}", status, error_text));
        }

        // 解析响应
        let response_text = response.text().await?;
        let json_value: Value = serde_json::from_str(&response_text)?;

        // 检查API响应状态
        if let Some(code) = json_value["code"].as_str() {
            if code != "0" {
                let msg = json_value["msg"].as_str().unwrap_or("未知错误");
                return Err(anyhow!("API错误: {} - {}", code, msg));
            }
        }

        // 解析响应数据
        let api_response: T = serde_json::from_value(json_value)?;
        Ok(api_response)
    }

    /// 生成签名
    fn generate_signature(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        timestamp: &str,
    ) -> Result<String> {
        // 构建签名字符串
        let mut sign_string = timestamp.to_string() + method + endpoint;

        // 添加查询参数
        if let Some(params) = params {
            if !params.is_empty() {
                let mut sorted_params: Vec<_> = params.iter().collect();
                sorted_params.sort_by_key(|&(k, _)| k);

                let param_string = sorted_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");

                sign_string.push_str("?");
                sign_string.push_str(&param_string);
            }
        }

        // 使用HMAC-SHA256生成签名
        let mut mac = Hmac::<Sha256>::new_from_slice(self.config.secret_key.as_bytes())
            .map_err(|e| anyhow!("HMAC初始化失败: {}", e))?;

        mac.update(sign_string.as_bytes());
        let result = mac.finalize();

        Ok(base64::encode(result.into_bytes()))
    }

    /// 获取时间戳
    async fn get_timestamp(&self) -> Result<String> {
        // 使用ISO 8601格式的时间戳，如：2020-12-08T09:08:57.715Z
        use chrono::Utc;
        let utc_now = Utc::now();
        // 格式化为ISO 8601格式，包含毫秒
        let timestamp = utc_now.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
        Ok(timestamp)
    }
}

/// 创建默认的API客户端
pub fn create_client(config: Config) -> OkxClient {
    OkxClient::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = Config {
            api_key: "test_key".to_string(),
            secret_key: "test_secret".to_string(),
            passphrase: "test_passphrase".to_string(),
            base_url: "https://www.okx.com".to_string(),
            is_sandbox: false,
        };

        let client = OkxClient::new(config);
        assert_eq!(client.config.api_key, "test_key");
    }

    #[test]
    fn test_signature_generation() {
        let config = Config {
            api_key: "test_key".to_string(),
            secret_key: "test_secret".to_string(),
            passphrase: "test_passphrase".to_string(),
            base_url: "https://www.okx.com".to_string(),
            is_sandbox: false,
        };

        let client = OkxClient::new(config);
        let timestamp = "2020-12-08T09:08:57.715Z";
        let method = "GET";
        let endpoint = "/api/v5/account/positions";
        let params = None;

        let signature = client.generate_signature(method, endpoint, params, timestamp).unwrap();
        assert!(!signature.is_empty());
    }
} 
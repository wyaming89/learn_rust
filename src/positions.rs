use crate::{
    config::Config,
    rate_limiter::RateLimiter,
    types::{ApiResponse, Position, PositionsParams},
};
use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use base64;

/// 获取当前持仓信息
pub async fn get_positions(
    config: &Config,
    params: &PositionsParams,
) -> Result<ApiResponse<Position>> {
    let client = Client::new();
    let rate_limiter = RateLimiter::new(10, std::time::Duration::from_secs(2));

    // 构建查询参数
    let mut query_params = HashMap::new();
    
    if let Some(inst_type) = &params.inst_type {
        query_params.insert("instType", inst_type);
    }
    
    if let Some(inst_id) = &params.inst_id {
        query_params.insert("instId", inst_id);
    }
    
    if let Some(pos_id) = &params.pos_id {
        query_params.insert("posId", pos_id);
    }

    // 构建URL
    let url = format!("{}/api/v5/account/positions", config.base_url);

    // 等待限速
    rate_limiter.wait_if_needed(&config.api_key)?;

    // 构建请求
    let mut request = client.get(&url);

    // 添加查询参数
    for (key, value) in &query_params {
        request = request.query(&[(key, value)]);
    }

    // 添加认证头
    let timestamp = get_timestamp().await?;
    let method = "GET";
    let request_path = "/api/v5/account/positions";
    
    // 构建签名字符串
    let mut sign_string = timestamp.clone();
    sign_string.push_str(method);
    sign_string.push_str(request_path);
    
    // 如果有查询参数，添加到签名字符串
    if !query_params.is_empty() {
        let mut sorted_params: Vec<_> = query_params.iter().collect();
        sorted_params.sort_by_key(|(k, _)| *k);
        
        let param_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        sign_string.push_str("?");
        sign_string.push_str(&param_string);
    }

    // 生成签名
    let signature = generate_signature(&config.secret_key, &sign_string)?;

    // 添加请求头
    request = request
        .header("OK-ACCESS-KEY", &config.api_key)
        .header("OK-ACCESS-SIGN", signature)
        .header("OK-ACCESS-TIMESTAMP", &timestamp)
        .header("OK-ACCESS-PASSPHRASE", &config.passphrase);

    if config.is_sandbox {
        request = request.header("x-simulated-trading", "1");
    }

    // 发送请求
    let response = request.send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("API请求失败: {}", error_text));
    }

    let response_text = response.text().await?;
    let json_value: Value = serde_json::from_str(&response_text)?;

    // 检查API响应状态
    if let Some(code) = json_value["code"].as_str() {
        if code != "0" {
            let msg = json_value["msg"].as_str().unwrap_or("未知错误");
            return Err(anyhow::anyhow!("API错误: {} - {}", code, msg));
        }
    }

    // 解析响应数据
    let api_response: ApiResponse<Position> = serde_json::from_value(json_value)?;
    Ok(api_response)
}

/// 生成签名
fn generate_signature(secret_key: &str, message: &str) -> Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .map_err(|e| anyhow::anyhow!("HMAC创建失败: {}", e))?;
    
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature = base64::encode(result.into_bytes());
    
    Ok(signature)
}

/// 获取时间戳
async fn get_timestamp() -> Result<String> {
    // 使用系统时间生成ISO 8601格式的时间戳
    let now = chrono::Utc::now();
    Ok(now.to_rfc3339())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions_params_serialization() {
        let params = PositionsParams {
            inst_type: Some("SWAP".to_string()),
            inst_id: Some("BTC-USD-SWAP".to_string()),
            pos_id: None,
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("SWAP"));
        assert!(json.contains("BTC-USD-SWAP"));
    }
} 
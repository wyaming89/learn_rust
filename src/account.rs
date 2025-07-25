use crate::{
    client::OkxClient,
    config::Config,
    types::ApiResponse,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 账户余额信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    /// 币种
    pub ccy: String,
    /// 可用余额
    #[serde(rename = "availBal")]
    pub avail_bal: String,
    /// 余额
    pub bal: String,
    /// 冻结余额
    #[serde(rename = "frozenBal")]
    pub frozen_bal: String,
    /// 账户类型
    #[serde(rename = "acctType")]
    pub acct_type: String,
}

/// 账户配置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountConfig {
    /// 账户ID
    #[serde(rename = "acctId")]
    pub acct_id: String,
    /// 账户类型
    #[serde(rename = "acctType")]
    pub acct_type: String,
    /// 币种
    pub ccy: String,
    /// 是否支持交易
    #[serde(rename = "tradingEnabled")]
    pub trading_enabled: bool,
    /// 是否支持提现
    #[serde(rename = "withdrawEnabled")]
    pub withdraw_enabled: bool,
    /// 是否支持充值
    #[serde(rename = "depositEnabled")]
    pub deposit_enabled: bool,
}

/// 获取账户余额
pub async fn get_account_balance(config: &Config) -> Result<ApiResponse<AccountBalance>> {
    let client = OkxClient::new(config.clone());
    
    // 执行API请求 - 不需要查询参数
    let response: ApiResponse<AccountBalance> = client
        .get("/api/v5/account/balance", None)
        .await?;

    Ok(response)
}

/// 获取账户配置
pub async fn get_account_config(config: &Config) -> Result<ApiResponse<AccountConfig>> {
    let client = OkxClient::new(config.clone());
    
    // 执行API请求 - 不需要查询参数
    let response: ApiResponse<AccountConfig> = client
        .get("/api/v5/account/config", None)
        .await?;

    Ok(response)
}

/// 获取指定币种的账户余额
pub async fn get_account_balance_by_currency(
    config: &Config,
    ccy: &str,
) -> Result<ApiResponse<AccountBalance>> {
    let client = OkxClient::new(config.clone());
    
    // 构建查询参数
    let mut query_params = HashMap::new();
    query_params.insert("ccy".to_string(), ccy.to_string());
    
    // 执行API请求
    let response: ApiResponse<AccountBalance> = client
        .get("/api/v5/account/balance", Some(&query_params))
        .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_balance_deserialization() {
        let json = r#"{
            "ccy": "BTC",
            "availBal": "1.0",
            "bal": "1.5",
            "frozenBal": "0.5",
            "acctType": "18"
        }"#;

        let balance: AccountBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.ccy, "BTC");
        assert_eq!(balance.avail_bal, "1.0");
        assert_eq!(balance.bal, "1.5");
        assert_eq!(balance.frozen_bal, "0.5");
        assert_eq!(balance.acct_type, "18");
    }

    #[test]
    fn test_account_config_deserialization() {
        let json = r#"{
            "acctId": "123456",
            "acctType": "18",
            "ccy": "BTC",
            "tradingEnabled": true,
            "withdrawEnabled": true,
            "depositEnabled": true
        }"#;

        let config: AccountConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.acct_id, "123456");
        assert_eq!(config.acct_type, "18");
        assert_eq!(config.ccy, "BTC");
        assert!(config.trading_enabled);
        assert!(config.withdraw_enabled);
        assert!(config.deposit_enabled);
    }
} 
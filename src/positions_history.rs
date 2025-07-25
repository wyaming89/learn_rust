use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

use crate::{
    client::OkxClient,
    config::Config,
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

/// 获取历史持仓信息
pub async fn get_positions_history(
    config: &Config,
    params: &PositionsHistoryParams,
) -> Result<ApiResponse<PositionHistory>> {
    let client = OkxClient::new(config.clone());
    
    // 构建查询参数
    let query_params = params.to_query_params();
    
    // 执行API请求
    let response: ApiResponse<PositionHistory> = client
        .get("/api/v5/account/positions-history", Some(&query_params))
        .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions_history_params_serialization() {
        let params = PositionsHistoryParams {
            inst_type: Some("SWAP".to_string()),
            inst_id: Some("BTC-USD-SWAP".to_string()),
            mgn_mode: Some("cross".to_string()),
            close_type: Some("2".to_string()),
            pos_id: None,
            before: None,
            after: None,
            limit: Some("50".to_string()),
        };

        let query_params = params.to_query_params();
        assert_eq!(query_params.get("instType"), Some(&"SWAP".to_string()));
        assert_eq!(query_params.get("instId"), Some(&"BTC-USD-SWAP".to_string()));
        assert_eq!(query_params.get("mgnMode"), Some(&"cross".to_string()));
        assert_eq!(query_params.get("type"), Some(&"2".to_string()));
        assert_eq!(query_params.get("limit"), Some(&"50".to_string()));
    }
} 
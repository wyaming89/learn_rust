use crate::{
    client::OkxClient,
    config::Config,
    types::{ApiResponse, Position, PositionsParams},
};
use anyhow::Result;
use std::collections::HashMap;

/// 获取当前持仓信息
pub async fn get_positions(
    config: &Config,
    params: &PositionsParams,
) -> Result<ApiResponse<Position>> {
    let client = OkxClient::new(config.clone());

    // 构建查询参数
    let mut query_params = HashMap::new();
    
    if let Some(inst_type) = &params.inst_type {
        query_params.insert("instType".to_string(), inst_type.clone());
    }
    
    if let Some(inst_id) = &params.inst_id {
        query_params.insert("instId".to_string(), inst_id.clone());
    }
    
    if let Some(pos_id) = &params.pos_id {
        query_params.insert("posId".to_string(), pos_id.clone());
    }

    // 执行API请求
    let response: ApiResponse<Position> = client
        .get("/api/v5/account/positions", Some(&query_params))
        .await?;

    Ok(response)
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
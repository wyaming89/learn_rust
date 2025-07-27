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

/// 获取简化的持仓信息
pub async fn get_positions_simplified(
    config: &Config,
    params: &PositionsParams,
) -> Result<Vec<SimplifiedPosition>> {
    let response = get_positions(config, params).await?;
    
    let simplified_positions: Vec<SimplifiedPosition> = response.data
        .into_iter()
        .map(|pos| SimplifiedPosition {
            pair: pos.inst_id,
            side: pos.pos_side,
            avail_pos: pos.pos,
            avg_px: pos.avg_px,
            mark_px: pos.mark_px,
            upl: pos.upl,
            upl_ratio: pos.upl_ratio,
        })
        .collect();
    
    Ok(simplified_positions)
}

/// 简化的持仓信息结构
#[derive(Debug, Clone)]
pub struct SimplifiedPosition {
    pub pair: String,
    pub side: String,
    pub avail_pos: String,
    pub avg_px: String,
    pub mark_px: String,
    pub upl: String,
    pub upl_ratio: String,
}

impl SimplifiedPosition {
    /// 格式化输出
    pub fn format_display(&self) -> String {
        format!(
            "{:<12} | {:<6} | {:<8} | {:<12} | {:<12} | {:<12} | {:<8}",
            self.pair,
            self.side,
            self.avail_pos,
            self.avg_px,
            self.mark_px,
            self.upl,
            self.upl_ratio
        )
    }
    
    /// JSON格式输出
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "Pair": self.pair,
            "Side": self.side,
            "availPos": self.avail_pos,
            "avgPx": self.avg_px,
            "markPx": self.mark_px,
            "upl": self.upl,
            "uplRatio": self.upl_ratio
        })
    }
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

    #[test]
    fn test_simplified_position_format() {
        let pos = SimplifiedPosition {
            pair: "BTC-USDT-SWAP".to_string(),
            side: "long".to_string(),
            avail_pos: "1.0".to_string(),
            avg_px: "50000.0".to_string(),
            mark_px: "51000.0".to_string(),
            upl: "1000.0".to_string(),
            upl_ratio: "0.02".to_string(),
        };

        let formatted = pos.format_display();
        assert!(formatted.contains("BTC-USDT-SWAP"));
        assert!(formatted.contains("long"));
        assert!(formatted.contains("1.0"));
    }
} 
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionHistory {
    /// 产品类型
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// 交易产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 保证金模式
    #[serde(rename = "mgnMode")]
    pub mgn_mode: String,
    /// 持仓ID
    #[serde(rename = "posId")]
    pub pos_id: String,
    /// 持仓方向
    #[serde(rename = "posSide")]
    pub pos_side: String,
    /// 开仓平均价
    #[serde(rename = "openAvgPx")]
    pub open_avg_px: String,
    /// 平仓平均价
    #[serde(rename = "closeAvgPx")]
    pub close_avg_px: String,
    /// 已实现收益
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: String,
    /// 平仓类型
    #[serde(rename = "type")]
    pub close_type: String,
    /// 开仓时间
    #[serde(rename = "cTime")]
    pub open_time: String,
    /// 仓位更新时间
    #[serde(rename = "uTime")]
    pub u_time: String,
    /// 杠杆倍数
    pub lever: String,
    /// 手续费
    pub fee: String,
    /// 资金费用
    #[serde(rename = "fundingFee")]
    pub funding_fee: String,
    /// 方向
    pub direction: String,
    /// 币种
    pub ccy: String,
    /// 平仓总持仓
    #[serde(rename = "closeTotalPos")]
    pub close_total_pos: String,
    /// 开仓最大持仓
    #[serde(rename = "openMaxPos")]
    pub open_max_pos: String,
    /// 盈亏
    pub pnl: String,
    /// 盈亏比例
    #[serde(rename = "pnlRatio")]
    pub pnl_ratio: String,
    /// 强平惩罚
    #[serde(rename = "liqPenalty")]
    pub liq_penalty: String,
    /// 非结算平均价格
    #[serde(rename = "nonSettleAvgPx")]
    pub non_settle_avg_px: String,
    /// 结算盈亏
    #[serde(rename = "settledPnl")]
    pub settled_pnl: String,
    /// 触发价格
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    /// 标的
    pub uly: String,
} 
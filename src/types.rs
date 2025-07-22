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

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
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
    /// 持仓数量
    pub pos: String,
    /// 开仓平均价
    #[serde(rename = "avgPx")]
    pub avg_px: String,
    /// 未实现收益
    #[serde(rename = "upl")]
    pub upl: String,
    /// 未实现收益率
    #[serde(rename = "uplRatio")]
    pub upl_ratio: String,
    /// 杠杆倍数
    pub lever: String,
    /// 标记价格
    #[serde(rename = "markPx")]
    pub mark_px: String,
    /// 最新成交价
    #[serde(rename = "lastPx")]
    pub last_px: String,
    /// 最新成交价时间
    #[serde(rename = "lastPxTime")]
    pub last_px_time: String,
    /// 持仓价值
    #[serde(rename = "posValue")]
    pub pos_value: String,
    /// 保证金
    pub margin: String,
    /// 保证金率
    #[serde(rename = "mgnRatio")]
    pub mgn_ratio: String,
    /// 维持保证金率
    #[serde(rename = "maintMarginRatio")]
    pub maint_margin_ratio: String,
    /// 强平价格
    #[serde(rename = "liqPx")]
    pub liq_px: String,
    /// 利息
    pub interest: String,
    /// 资金费用
    #[serde(rename = "fundingFee")]
    pub funding_fee: String,
    /// 资金费用时间
    #[serde(rename = "fundingTime")]
    pub funding_time: String,
    /// 方向
    pub direction: String,
    /// 币种
    pub ccy: String,
    /// 持仓时间
    #[serde(rename = "cTime")]
    pub c_time: String,
    /// 仓位更新时间
    #[serde(rename = "uTime")]
    pub u_time: String,
    /// 标的
    pub uly: String,
    /// 期权价值
    #[serde(rename = "optVal")]
    pub opt_val: Option<String>,
    /// 期权价值时间
    #[serde(rename = "optValTime")]
    pub opt_val_time: Option<String>,
    /// 期权价值标记价格
    #[serde(rename = "optValMarkPx")]
    pub opt_val_mark_px: Option<String>,
    /// 期权价值最新价格
    #[serde(rename = "optValLastPx")]
    pub opt_val_last_px: Option<String>,
    /// 期权价值最新价格时间
    #[serde(rename = "optValLastPxTime")]
    pub opt_val_last_px_time: Option<String>,
    /// 期权价值持仓价值
    #[serde(rename = "optValPosValue")]
    pub opt_val_pos_value: Option<String>,
    /// 期权价值保证金
    #[serde(rename = "optValMargin")]
    pub opt_val_margin: Option<String>,
    /// 期权价值保证金率
    #[serde(rename = "optValMgnRatio")]
    pub opt_val_mgn_ratio: Option<String>,
    /// 期权价值维持保证金率
    #[serde(rename = "optValMaintMarginRatio")]
    pub opt_val_maint_margin_ratio: Option<String>,
    /// 期权价值强平价格
    #[serde(rename = "optValLiqPx")]
    pub opt_val_liq_px: Option<String>,
    /// 期权价值利息
    #[serde(rename = "optValInterest")]
    pub opt_val_interest: Option<String>,
    /// 期权价值资金费用
    #[serde(rename = "optValFundingFee")]
    pub opt_val_funding_fee: Option<String>,
    /// 期权价值资金费用时间
    #[serde(rename = "optValFundingTime")]
    pub opt_val_funding_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionsParams {
    /// 产品类型
    #[serde(rename = "instType")]
    pub inst_type: Option<String>,
    /// 交易产品ID
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// 持仓ID
    #[serde(rename = "posId")]
    pub pos_id: Option<String>,
} 
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub trait MerakStrategy {
    fn on_start() -> Result<(), String> { Ok(()) }
    fn on_kline(_kline: Kline) -> Result<(), String> { Ok(()) }
}

/// OHLCV K线数据结构
/// 这个定义必须与 strategy.wit 中的 record kline 保持字段一一对应
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")] // 建议与前端/API保持一致的命名风格
pub struct Kline {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: u64,
}

// --- 3. 预导出的常用项 (Prelude) ---
pub mod prelude {
    pub use crate::{MerakStrategy, Kline};
    pub use serde::{Serialize, Deserialize};
    pub use schemars::JsonSchema;
}
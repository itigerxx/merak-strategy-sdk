// 1. 生成绑定
// 我们不指定具体的生成模式，让它根据 target 自动适配
wit_bindgen::generate!({
    path: "./wit/strategy.wit",
});

// 2. 统一导出类型
// 这样用户只需要 use merak_sdk::*; 就能拿到所有东西
pub use exports::merak::strategy::strategy::{Kline, Platform, Guest};

/// 辅助宏：为了让用户在 strategy.rs 里写代码更方便
/// 我们把常用的 derive 宏也通过 SDK 暴露出去
pub mod prelude {
    pub use serde::{Serialize, Deserialize};
    pub use schemars::JsonSchema;
    pub use crate::{Kline, Platform, Guest};
}
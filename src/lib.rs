wit_bindgen::generate!({
    path: "./wit/strategy.wit",
    // 关键：这里只生成类型定义，不生成导出逻辑
});

// 2. 重新导出常用类型，方便用户引用
pub use exports::merak::strategy::strategy::{Kline, Platform, Guest};
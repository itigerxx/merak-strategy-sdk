pub mod bindings {
    // 整个项目唯一的 generate!
    wit_bindgen::generate!({
        path: "./wit/strategy.wit",
        world: "quant",
        pub_export_macro: true, // 明确要求导出宏
        export_macro_name: "export_strategy", // 给宏起个独一无二的名字，防止冲突
    });
}

#[doc(inline)]
pub use bindings::export_strategy;

pub use bindings::merak::strategy::types::*;
pub use bindings::merak::strategy::platform::Platform;

use std::sync::OnceLock;

// 全局上下文存储
pub static PLATFORM: OnceLock<Platform> = OnceLock::new();
pub static CONFIG_STR: OnceLock<String> = OnceLock::new();

pub trait MerakStrategy {
    fn on_start() -> Result<(), String>;
    fn on_kline(kline: Kline) -> Result<(), String>;
}

/// 供 backend 调用，初始化上下文
pub fn __init_context(p: Platform, config: String) {
    let _ = PLATFORM.set(p);
    let _ = CONFIG_STR.set(config);
}

// --- 用户工具函数 ---

pub fn log(msg: &str) {
    if let Some(p) = PLATFORM.get() {
        p.log(msg);
    }
}

pub fn read_config<T: serde::de::DeserializeOwned>() -> Result<T, String> {
    let raw = CONFIG_STR.get().ok_or("Config not initialized")?;
    serde_json::from_str(raw).map_err(|e| format!("JSON Parse Error: {}", e))
}

pub fn place_order(req: OrderRequest) -> String {
    PLATFORM.get().map(|p| p.place_order(&req)).unwrap_or_default()
}
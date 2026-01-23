use std::sync::OnceLock;

// 1. 生成所有 WIT 类型和宏
pub mod bindings {
    wit_bindgen::generate!({
        path: "./wit/strategy.wit",
        world: "quant",
        pub_export_macro: true,
    });
}

// 2. 重新导出常用类型给用户
pub use bindings::merak::strategy::types::*;
pub use bindings::merak::strategy::platform::Platform;

// 3. 静态上下文存储
pub static PLATFORM: OnceLock<Box<dyn std::any::Any + Send + Sync>> = OnceLock::new();
pub static CONFIG_STR: OnceLock<String> = OnceLock::new();

// 4. 用户策略必须实现的 Trait
pub trait MerakStrategy {
    fn on_start() -> Result<(), String>;
    fn on_kline(kline: Kline) -> Result<(), String>;
}

// 5. 初始化与配置工具
pub fn __init_context<P>(p: P, config: String) where P: Send + Sync + 'static {
    let _ = PLATFORM.set(Box::new(p));
    let _ = CONFIG_STR.set(config);
}

pub fn read_config<T: serde::de::DeserializeOwned>() -> Result<T, String> {
    let raw = CONFIG_STR.get().ok_or("Config not init")?;
    serde_json::from_str(raw).map_err(|e| e.to_string())
}
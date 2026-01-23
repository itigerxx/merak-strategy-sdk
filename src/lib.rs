pub mod bindings {
    wit_bindgen::generate!({
        path: "./wit/strategy.wit",
        world: "quant",
        pub_export_macro: true,
    });
}

// 必须拉平路径，否则 Template 的 with 映射不到
pub use bindings::merak::strategy::types as types;
pub use bindings::merak::strategy::platform as platform;
pub use types::Kline;
pub use platform::Platform;

pub trait MerakStrategy {
    fn on_start() -> Result<(), String>;
    fn on_kline(kline: Kline) -> Result<(), String>;
}

use std::sync::OnceLock;
pub static PLATFORM: OnceLock<Box<dyn std::any::Any + Send + Sync>> = OnceLock::new();
pub static CONFIG_STR: OnceLock<String> = OnceLock::new();

pub fn __init_context<P>(p: P, config: String) where P: Send + Sync + 'static {
    let _ = PLATFORM.set(Box::new(p));
    let _ = CONFIG_STR.set(config);
}

pub fn read_config<T: serde::de::DeserializeOwned>() -> Result<T, String> {
    let raw = CONFIG_STR.get().ok_or("Config not init")?;
    serde_json::from_str(raw).map_err(|e| e.to_string())
}
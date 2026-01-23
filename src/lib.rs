// merak-strategy-sdk/src/lib.rs
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};
use std::sync::{OnceLock, RwLock};

// 1. 生成 WIT 绑定（包含 Kline 和 Platform 定义）
wit_bindgen::generate!({
    path: "./wit/strategy.wit", // 注意确保路径正确
});

// 重新导出生成的类型
pub use crate::merak::strategy::platform::Platform;
pub use crate::merak::strategy::types::Kline;

/// 策略必须实现的接口
pub trait MerakStrategy {
    fn on_start() -> Result<(), String> {
        Ok(())
    }
    fn on_kline(kline: Kline) -> Result<(), String>;
}

// --- 全局上下文管理 ---
static PLATFORM: OnceLock<Platform> = OnceLock::new();
static CONFIG_JSON: OnceLock<RwLock<String>> = OnceLock::new();

/// 获取平台 API 句柄
pub fn platform() -> &'static Platform {
    PLATFORM.get().expect("SDK: Platform 未初始化")
}

/// 泛型配置读取：自动将缓存的 JSON 转为用户定义的 Config 结构体
pub fn read_config<T: serde::de::DeserializeOwned>() -> Result<T, String> {
    let json_lock = CONFIG_JSON.get().ok_or("SDK: Config 未初始化")?;
    let json = json_lock.read().map_err(|_| "SDK: 读锁冲突")?;
    serde_json::from_str(&json).map_err(|e| format!("配置解析失败: {}", e))
}

/// 供 Template 初始化调用（隐藏 API）
#[doc(hidden)]
pub fn __init_context(p: Platform, json: String) {
    let _ = PLATFORM.set(p);
    let _ = CONFIG_JSON.set(RwLock::new(json));
}

pub mod prelude {
    pub use crate::{Kline, MerakStrategy, platform, read_config};
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}

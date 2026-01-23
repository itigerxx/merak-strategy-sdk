use std::sync::OnceLock;

// 1. 静态上下文存储
pub static PLATFORM: OnceLock<Box<dyn std::any::Any + Send + Sync>> = OnceLock::new();
pub static CONFIG_STR: OnceLock<String> = OnceLock::new();

// 2. 供 Backend 调用的初始化入口
pub fn __init_context<P>(p: P, config: String) where P: Send + Sync + 'static {
    let _ = PLATFORM.set(Box::new(p));
    let _ = CONFIG_STR.set(config);
}

// 3. 策略必须实现的接口 (泛型 TKline 为了兼容未来不同的 WIT 类型)
pub trait MerakStrategy<TKline> {
    fn on_start() -> Result<(), String>;
    fn on_kline(kline: TKline) -> Result<(), String>;
}

// 4. 基础 JSON 工具
pub fn read_config<T: serde::de::DeserializeOwned>() -> Result<T, String> {
    let raw = CONFIG_STR.get().ok_or("Config not init")?;
    serde_json::from_str(raw).map_err(|e| e.to_string())
}
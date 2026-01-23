pub mod bindings {
    wit_bindgen::generate!({
        path: "./wit/strategy.wit",
        world: "quant",
        // 0.51 增强了宏的导出能力
        pub_export_macro: true,
    });
}

// 导出给用户直接用
pub use bindings::merak::strategy::types::*;
pub use bindings::merak::strategy::platform::Platform;

pub trait MerakStrategy {
    fn on_start() -> Result<(), String>;
    fn on_kline(kline: Kline) -> Result<(), String>;
}
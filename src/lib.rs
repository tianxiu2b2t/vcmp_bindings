/// bindgen 生成的东西还是放在 raw 里把
///
/// bindgen .\c_src\plugin.h -o .\rs_src\bindings\raw.rs  --no-layout-tests --allowlist-item="(vcmp|Server|Plugin|PLUGIN).*"
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod raw;

/// gbk <-> utf8
pub mod encodes;
/// vcmp error & vcmp result
///
/// error wrapper
pub mod error;
/// callback event
pub mod events;
/// PluginFunction 的包装
///
/// 帮你解决好各种 call 问题
pub mod func;
/// wrapper for option enums
pub mod options;
/// wrapper for PluginInfo
pub mod plugin_info;
/// wrapper for PluginSetting
pub mod setting;
pub mod states;
/// 类型 wrapper
pub mod types;
pub mod utils;

pub use error::{VcmpError, VcmpResult};
pub use func::VcmpFunctions;
pub use plugin_info::VcmpPluginInfo;
pub use types::{CheckpointId, MarkerId, ObjectId, PlayerId, RadioId, VehicleId, WeaponId};

use std::sync::OnceLock;

/// 全局的 VcmpFunctions
pub static VCMP_FUNC: OnceLock<VcmpFunctions> = OnceLock::new();

pub fn init_vcmp_func(func: VcmpFunctions) -> &'static VcmpFunctions {
    VCMP_FUNC.get_or_init(|| func)
}

pub fn is_initialized_func() -> bool {
    VCMP_FUNC.get().is_some()
}

pub fn vcmp_func() -> &'static VcmpFunctions {
    VCMP_FUNC.get().unwrap()
}

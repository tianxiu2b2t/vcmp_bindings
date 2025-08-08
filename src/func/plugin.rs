use crate::{VcmpError, VcmpPluginInfo, VcmpResult, func::VcmpFunctions};

#[derive(Debug, Clone)]
pub struct PluginExports {
    pub exports_ptr: *mut *const std::os::raw::c_void,
    pub size: usize,
}

pub trait PluginMethods {
    /// 获取插件(加载?)数量
    fn get_plugin_count(&self) -> u32;

    /// 获取插件信息
    fn get_plugin_info(&self, id: i32) -> Option<VcmpPluginInfo>;

    /// 查找插件的 id
    fn find_plugin(&self, plugin_name: &str) -> Option<i32>;

    fn send_plugin_command(&self, command_identifier: u32, command: &str) -> VcmpResult<()>;

    fn get_plugin_exports(&self, plugin_id: i32) -> PluginExports;

    fn get_plugins(&self) -> Vec<VcmpPluginInfo>;
}

impl PluginMethods for VcmpFunctions {
    /// 获取插件(加载?)数量
    fn get_plugin_count(&self) -> u32 {
        (self.inner.GetNumberOfPlugins)()
    }

    /// 获取插件信息
    fn get_plugin_info(&self, plugin_id: i32) -> Option<VcmpPluginInfo> {
        let mut info = VcmpPluginInfo::new_empty();
        let info_ptr = info.inner_mut_ptr();
        let code = (self.inner.GetPluginInfo)(plugin_id, info_ptr);
        if code != 0 { None } else { Some(info) }
    }
    /// 查找插件的 id
    fn find_plugin(&self, plugin_name: &str) -> Option<i32> {
        let name = format!("{plugin_name}\0");
        let ptr = name.as_ptr() as *const i8;
        let res = (self.inner.FindPlugin)(ptr);
        if res == -1 { None } else { Some(res) }
    }

    fn send_plugin_command(&self, command_identifier: u32, command: &str) -> VcmpResult<()> {
        let cmd = format!("{command}\0");
        let cmd_ptr = cmd.as_ptr() as *const i8;
        let code = (self.inner.SendPluginCommand)(command_identifier, cmd_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_plugin_exports(&self, plugin_id: i32) -> PluginExports {
        let mut size: usize = 0;
        let ptr = (self.inner.GetPluginExports)(plugin_id, &mut size);
        PluginExports {
            exports_ptr: ptr,
            size,
        }
    }

    fn get_plugins(&self) -> Vec<VcmpPluginInfo> {
        (0..self.get_plugin_count())
            .filter_map(|id| self.get_plugin_info(id as i32))
            .collect::<Vec<_>>()
    }
}

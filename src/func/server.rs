use crate::encodes::{decode_gbk, encode_to_gbk};
use crate::func::VcmpFunctions;
use crate::setting::VcmpServerSettings;
use crate::{VcmpError, VcmpResult};

pub trait ServerMethods {
    fn set_server_name(&self, name: &str) -> VcmpResult<()>;
    fn set_server_password(&self, password: &str) -> VcmpResult<()>;
    fn set_gamemode(&self, gamemode: &str) -> VcmpResult<()>;
    fn get_max_players(&self) -> u32;
    fn shutdown(&self);
    fn server_version(&self) -> u32;
    fn server_settings(&self) -> VcmpServerSettings;
    fn get_server_name(&self) -> String;
    fn get_server_password(&self) -> String;
    fn get_gamemode(&self) -> String;
    fn set_max_players(&self, max_player: u32) -> VcmpResult<()>;
}

impl ServerMethods for VcmpFunctions {
    fn set_server_name(&self, name: &str) -> VcmpResult<()> {
        let mut name = encode_to_gbk(name).to_vec();
        name.push(0); // 到 C 层面要加一个 \0
        let name_ptr = name.as_ptr() as *const i8;
        let code = (self.inner.SetServerName)(name_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_server_password(&self, password: &str) -> VcmpResult<()> {
        let mut password = encode_to_gbk(password).to_vec();
        password.push(0); // 到 C 层面要加一个 \0
        let password_ptr = password.as_ptr() as *const i8;
        let code = (self.inner.SetServerPassword)(password_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_gamemode(&self, gamemode: &str) -> VcmpResult<()> {
        let mut gamemode = encode_to_gbk(gamemode).to_vec();
        gamemode.push(0); // 到 C 层面要加一个 \0
        let gamemode_ptr = gamemode.as_ptr() as *const i8;
        let code = (self.inner.SetGameModeText)(gamemode_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn get_max_players(&self) -> u32 {
        (self.inner.GetMaxPlayers)()
    }
    fn shutdown(&self) {
        (self.inner.ShutdownServer)()
    }
    /// 获取服务器版本
    fn server_version(&self) -> u32 {
        (self.inner.GetServerVersion)()
    }
    /// 获取服务器设置
    fn server_settings(&self) -> VcmpServerSettings {
        let mut setting = VcmpServerSettings::new_empty();
        let setting_ptr = setting.inner_mut_ptr();
        let _ = (self.inner.GetServerSettings)(setting_ptr);
        setting
    }
    fn get_server_name(&self) -> String {
        self.server_settings()
            .server_name()
            .trim_end_matches("\0")
            .to_string()
    }
    fn get_server_password(&self) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetServerPassword)(buf_ptr, 1024);
        decode_gbk(&buf).trim_end_matches("\0").to_string()
    }
    fn get_gamemode(&self) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetGameModeText)(buf_ptr, 1024);
        decode_gbk(&buf).trim_end_matches("\0").to_string()
    }
    fn set_max_players(&self, max_player: u32) -> VcmpResult<()> {
        let code = (self.inner.SetMaxPlayers)(max_player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}

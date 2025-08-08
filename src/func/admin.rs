use crate::func::VcmpFunctions;

pub trait AdministrationMethods {
    fn ban_ip(&self, ip: &str);
    fn unban_ip(&self, ip: &str) -> bool;
    fn is_ip_banned(&self, ip: &str) -> bool;
}

impl AdministrationMethods for VcmpFunctions {
    fn ban_ip(&self, ip: &str) {
        let addr = format!("{ip}\0");
        (self.inner.BanIP)(addr.as_ptr() as *mut i8);
    }
    fn unban_ip(&self, ip: &str) -> bool {
        let addr = format!("{ip}\0");
        (self.inner.UnbanIP)(addr.as_ptr() as *mut i8) != 0
    }
    fn is_ip_banned(&self, ip: &str) -> bool {
        let addr = format!("{ip}\0");
        (self.inner.IsIPBanned)(addr.as_ptr() as *mut i8) != 0
    }
}

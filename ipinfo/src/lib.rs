#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::cell::UnsafeCell;
use core::fmt::{Display, Formatter};
use indoc::writedoc;
use json::Value;
use obfstr::obfstr as s;
use requests::{Request, RequestBuilder, ResponseBodyExt};
use utils::internal_code_to_flag;

static mut GLOBAL_IP_INFO: UnsafeCell<Option<IpInfo>> = UnsafeCell::new(None);

pub struct IpInfo {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
}

impl Display for IpInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writedoc!(
            f,
            "
            IP: {}
            \tCity:\t({}) {}
            \tRegion:\t{}
            \tPostal:\t{}",
            self.ip,
            internal_code_to_flag(&self.country).unwrap_or_else(|| self.country.clone()),
            self.city,
            self.region,
            self.postal
        )
    }
}

#[allow(static_mut_refs)]
pub fn get_ip_info() -> Option<&'static IpInfo> {
    unsafe {
        let ip_info = &*GLOBAL_IP_INFO.get();
        ip_info.as_ref()
    }
}

pub fn unwrapped_ip_info() -> &'static IpInfo {
    get_ip_info().unwrap()
}

impl IpInfo {
    fn from_value(value: Value) -> Option<Self> {
        let ip: String = value.get(s!("ip"))?.as_string()?.to_owned();
        let city = value.get(s!("city"))?.as_string()?.to_owned();
        let region = value.get(s!("region"))?.as_string()?.to_owned();
        let country = value.get(s!("country"))?.as_string()?.to_owned();
        let loc = value.get(s!("loc"))?.as_string()?.to_owned();
        let org = value.get(s!("org"))?.as_string()?.to_owned();
        let postal = value.get(s!("postal"))?.as_string()?.to_owned();
        let timezone = value.get(s!("timezone"))?.as_string()?.to_owned();

        Some(Self {
            ip,
            city,
            region,
            country,
            loc,
            org,
            postal,
            timezone,
        })
    }
}

impl TryFrom<Value> for IpInfo {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(())
    }
}

#[allow(static_mut_refs)]
pub fn init_ip_info() -> bool {
    if !get_ip_info().is_none() {
        return false;
    }

    let result = Request::get("https://ipinfo.io/json").build().send();

    let Some(json) = result
        .ok()
        .and_then(|response| response.body().as_json().ok())
    else {
        return false;
    };

    let Ok(info) = IpInfo::try_from(json) else {
        return false;
    };

    let slot = unsafe { &mut *GLOBAL_IP_INFO.get() };
    *slot = Some(info);

    true
}

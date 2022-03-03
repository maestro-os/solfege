//! This module implements a call to uname. Allowing to retrieve various informations.

use std::mem;

/// Structure containing the final informations to be returned outside of this module.
#[derive(Debug)]
pub struct UnameInfo {
    /// Operating system name
    pub sysname: String,
    /// Name within "some implementation-defined network"
    pub nodename: String,
    /// Operating system release
    pub release: String,
    /// Operating system version
    pub version: String,
    /// Hardware identifier
    pub machine: String,
    /// NIS or YP domain name
    pub domainname: String,
}

impl UnameInfo {
    /// Returns the uname informations.
    /// If the uname informations cannot be retrieved, the function returns an error.
    pub fn get() -> Result<Self, ()> {
        let mut uname_info = unsafe {
            mem::zeroed()
        };
        let result = unsafe {
            libc::uname(&mut uname_info)
        };

        if result == 0 {
            // TODO
            todo!();
        } else {
            Err(())
        }
    }
}

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_uint;

// Defined as an enum without specified values, so bindgen doesn't generate these.
// However, we can know the values per <https://stackoverflow.com/questions/6434105/are-default-enum-values-in-c-the-same-for-all-compilers>
pub const EVDI_STATUS_AVAILABLE: c_uint = 0;
pub const EVDI_STATUS_UNRECOGNIZED: c_uint = 1;
pub const EVDI_STATUS_NOT_PRESENT: c_uint = 2;

include!("./bindings.rs");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evdi_check_device_for_not_present() {
        let status = unsafe {
            evdi_check_device(42)
        };
        assert_eq!(status, EVDI_STATUS_NOT_PRESENT)
    }

    #[test]
    fn is_correct_version() {
        let mut version = evdi_lib_version {
            version_major: -1,
            version_minor: -1,
            version_patchlevel: -1
        };

        unsafe {
            evdi_get_lib_version(&mut version)
        }

        assert_eq!(version.version_major, 1);
        assert_eq!(version.version_minor, 9);
        assert_eq!(version.version_patchlevel, 1);
    }
}

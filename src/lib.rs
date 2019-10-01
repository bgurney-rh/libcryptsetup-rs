#![deny(missing_docs)]

//! This is a wrapper library for libcryptsetup. The intension is to provide as much safety as
//! possible when crossing FFI boundaries to the crypsetup C library.

// Keyfile reading functions are not supported in these bindings due
// to how memory is handled in these functions - memory for keys is allocated
// and the corresponding free functions are not part of the public API.
// This means that the memory cannot be safe scrubbed and freed in longer running
// processes that invoke this function. For now, this is disabled.

#[macro_use]
extern crate cryptsetup_cli_proc_macro;
extern crate cryptsetup_sys;
extern crate libc;
extern crate serde_json;
extern crate uuid;

use std::os::raw::c_int;

#[macro_use]
mod macros;

mod activate;
pub use activate::CryptActivation;

mod backup;
pub use backup::CryptBackup;

mod context;
pub use context::CryptContext;

mod debug;
pub use debug::CryptDebug;

mod device;
pub use device::{CryptDevice, CryptInit};

mod err;
pub use err::LibcryptErr;

mod format;
pub use format::CryptFormat;

mod key;
pub use key::CryptVolumeKey;

mod keyslot;
pub use keyslot::CryptKeyslot;

mod log;
pub use log::{CryptLog, CryptLogLevel};

mod luks2_flags;
pub use luks2_flags::{CryptLuks2Flags, CryptRequirementFlag, CryptRequirementFlags};

mod luks2_reencrypt;
pub use luks2_reencrypt::{
    CryptLuks2Reencrypt, CryptReencryptDirectionInfo, CryptReencryptFlag, CryptReencryptFlags,
    CryptReencryptModeInfo,
};

mod luks2_token;
pub use luks2_token::CryptLuks2Token;

mod runtime;
pub use runtime::CryptRuntime;

mod settings;
pub use settings::CryptSettings;

mod status;
pub use status::CryptDeviceStatus;

mod wipe;
pub use wipe::CryptWipe;

/// Re-export of `libc::size_t`
pub type SizeT = libc::size_t;

/// Boolean specifying yes or no
#[derive(Debug, Eq, PartialEq)]
pub enum Bool {
    /// False
    No = 0,
    /// True
    Yes = 1,
}

impl From<c_int> for Bool {
    fn from(v: c_int) -> Self {
        match v {
            i if i == 0 => Bool::No,
            _ => Bool::Yes,
        }
    }
}

/// Boolean specifying yes or no
#[derive(Debug, Eq, PartialEq)]
pub enum Interrupt {
    /// False
    No = Bool::No as isize,
    /// True
    Yes = Bool::Yes as isize,
}

impl From<c_int> for Interrupt {
    fn from(v: c_int) -> Self {
        match v {
            i if i == 0 => Interrupt::No,
            _ => Interrupt::Yes,
        }
    }
}

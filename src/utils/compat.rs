use uapi::c;

#[cfg(target_env = "musl")]
pub type IoctlNumber = c::c_int;

#[cfg(not(target_env = "musl"))]
pub type IoctlNumber = c::c_ulong;

#[cfg(target_env = "musl")]
pub type IovLength = c::c_int;

#[cfg(not(target_env = "musl"))]
pub type IovLength = usize;

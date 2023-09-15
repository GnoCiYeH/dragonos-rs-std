//! Platform-specific extensions to `std` for the WebAssembly System Interface (WASI).
//!
//! Provides access to platform-level information on WASI, and exposes
//! WASI-specific functions that would otherwise be inappropriate as
//! part of the core `std` library.
//!
//! It exposes more ways to deal with platform-specific strings (`OsStr`,
//! `OsString`), allows to set permissions more granularly, extract low-level
//! file descriptors from files and sockets, and has platform-specific helpers
//! for spawning processes.
//!
//! # Examples
//!
//! ```no_run
//! use std::fs::File;
//! use std::os::wasi::prelude::*;
//!
//! fn main() -> std::io::Result<()> {
//!     let f = File::create("foo.txt")?;
//!     let fd = f.as_raw_fd();
//!
//!     // use fd with native WASI bindings
//!
//!     Ok(())
//! }
//! ```
//!
//! [`OsStr`]: crate::std::ffi::OsStr
//! [`OsString`]: crate::std::ffi::OsString

#![deny(unsafe_op_in_unsafe_fn)]
#![doc(cfg(target_os = "wasi"))]

pub mod ffi;
pub mod fs;
pub mod io;
pub mod net;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
pub mod prelude {
    #[doc(no_inline)]
        pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)]
        pub use super::fs::FileTypeExt;
    #[doc(no_inline)]
        pub use super::fs::{DirEntryExt, FileExt, MetadataExt, OpenOptionsExt};
    #[doc(no_inline)]
        pub use super::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
}

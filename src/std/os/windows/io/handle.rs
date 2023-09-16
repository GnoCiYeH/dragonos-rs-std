//! Owned and borrowed OS handles.


use super::raw::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
use crate::std::fmt;
use crate::std::fs;
use crate::std::io;
use crate::std::marker::PhantomData;
use crate::std::mem::forget;
use crate::std::ptr;
use crate::std::sys;
use crate::std::sys::cvt;
use crate::std::sys_common::{AsInner, FromInner, IntoInner};

/// A borrowed handle.
///
/// This has a lifetime parameter to tie it to the lifetime of something that
/// owns the handle.
///
/// This uses `repr(transparent)` and has the representation of a host handle,
/// so it can be used in FFI in places where a handle is passed as an argument,
/// it is not captured or consumed.
///
/// Note that it *may* have the value `-1`, which in `BorrowedHandle` always
/// represents a valid handle value, such as [the current process handle], and
/// not `INVALID_HANDLE_VALUE`, despite the two having the same value. See
/// [here] for the full story.
///
/// And, it *may* have the value `NULL` (0), which can occur when consoles are
/// detached from processes, or when `windows_subsystem` is used.
///
/// This type's `.to_owned()` implementation returns another `BorrowedHandle`
/// rather than an `OwnedHandle`. It just makes a trivial copy of the raw
/// handle, which is then borrowed under the same lifetime.
///
/// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
/// [the current process handle]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess#remarks
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BorrowedHandle<'handle> {
    handle: RawHandle,
    _phantom: PhantomData<&'handle OwnedHandle>,
}

/// An owned handle.
///
/// This closes the handle on drop.
///
/// Note that it *may* have the value `-1`, which in `OwnedHandle` always
/// represents a valid handle value, such as [the current process handle], and
/// not `INVALID_HANDLE_VALUE`, despite the two having the same value. See
/// [here] for the full story.
///
/// And, it *may* have the value `NULL` (0), which can occur when consoles are
/// detached from processes, or when `windows_subsystem` is used.
///
/// `OwnedHandle` uses [`CloseHandle`] to close its handle on drop. As such,
/// it must not be used with handles to open registry keys which need to be
/// closed with [`RegCloseKey`] instead.
///
/// [`CloseHandle`]: https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
/// [`RegCloseKey`]: https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey
///
/// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
/// [the current process handle]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess#remarks
#[repr(transparent)]
pub struct OwnedHandle {
    handle: RawHandle,
}

/// FFI type for handles in return values or out parameters, where `NULL` is used
/// as a sentry value to indicate errors, such as in the return value of `CreateThread`. This uses
/// `repr(transparent)` and has the representation of a host handle, so that it can be used in such
/// FFI declarations.
///
/// The only thing you can usefully do with a `HandleOrNull` is to convert it into an
/// `OwnedHandle` using its [`TryFrom`] implementation; this conversion takes care of the check for
/// `NULL`. This ensures that such FFI calls cannot start using the handle without
/// checking for `NULL` first.
///
/// This type may hold any handle value that [`OwnedHandle`] may hold. As with `OwnedHandle`, when
/// it holds `-1`, that value is interpreted as a valid handle value, such as
/// [the current process handle], and not `INVALID_HANDLE_VALUE`.
///
/// If this holds a non-null handle, it will close the handle on drop.
///
/// [the current process handle]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess#remarks
#[repr(transparent)]
#[derive(Debug)]
pub struct HandleOrNull(OwnedHandle);

/// FFI type for handles in return values or out parameters, where `INVALID_HANDLE_VALUE` is used
/// as a sentry value to indicate errors, such as in the return value of `CreateFileW`. This uses
/// `repr(transparent)` and has the representation of a host handle, so that it can be used in such
/// FFI declarations.
///
/// The only thing you can usefully do with a `HandleOrInvalid` is to convert it into an
/// `OwnedHandle` using its [`TryFrom`] implementation; this conversion takes care of the check for
/// `INVALID_HANDLE_VALUE`. This ensures that such FFI calls cannot start using the handle without
/// checking for `INVALID_HANDLE_VALUE` first.
///
/// This type may hold any handle value that [`OwnedHandle`] may hold, except that when it holds
/// `-1`, that value is interpreted to mean `INVALID_HANDLE_VALUE`.
///
/// If holds a handle other than `INVALID_HANDLE_VALUE`, it will close the handle on drop.
#[repr(transparent)]
#[derive(Debug)]
pub struct HandleOrInvalid(OwnedHandle);

// The Windows [`HANDLE`] type may be transferred across and shared between
// thread boundaries (despite containing a `*mut void`, which in general isn't
// `Send` or `Sync`).
//
// [`HANDLE`]: std::os::windows::raw::HANDLE
unsafe impl Send for OwnedHandle {}
unsafe impl Send for HandleOrNull {}
unsafe impl Send for HandleOrInvalid {}
unsafe impl Send for BorrowedHandle<'_> {}
unsafe impl Sync for OwnedHandle {}
unsafe impl Sync for HandleOrNull {}
unsafe impl Sync for HandleOrInvalid {}
unsafe impl Sync for BorrowedHandle<'_> {}

impl BorrowedHandle<'_> {
    /// Return a `BorrowedHandle` holding the given raw handle.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `handle` must be a valid open handle, it
    /// must remain open for the duration of the returned `BorrowedHandle`.
    ///
    /// Note that it *may* have the value `INVALID_HANDLE_VALUE` (-1), which is
    /// sometimes a valid handle value. See [here] for the full story.
    ///
    /// And, it *may* have the value `NULL` (0), which can occur when consoles are
    /// detached from processes, or when `windows_subsystem` is used.
    ///
    /// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
    #[inline]
            pub const unsafe fn borrow_raw(handle: RawHandle) -> Self {
        Self { handle, _phantom: PhantomData }
    }
}

impl TryFrom<HandleOrNull> for OwnedHandle {
    type Error = NullHandleError;

    #[inline]
    fn try_from(handle_or_null: HandleOrNull) -> Result<Self, NullHandleError> {
        let owned_handle = handle_or_null.0;
        if owned_handle.handle.is_null() {
            // Don't call `CloseHandle`; it'd be harmless, except that it could
            // overwrite the `GetLastError` error.
            forget(owned_handle);

            Err(NullHandleError(()))
        } else {
            Ok(owned_handle)
        }
    }
}

impl OwnedHandle {
    /// Creates a new `OwnedHandle` instance that shares the same underlying
    /// object as the existing `OwnedHandle` instance.
        pub fn try_clone(&self) -> crate::std::io::Result<Self> {
        self.as_handle().try_clone_to_owned()
    }
}

impl BorrowedHandle<'_> {
    /// Creates a new `OwnedHandle` instance that shares the same underlying
    /// object as the existing `BorrowedHandle` instance.
        pub fn try_clone_to_owned(&self) -> crate::std::io::Result<OwnedHandle> {
        self.duplicate(0, false, sys::c::DUPLICATE_SAME_ACCESS)
    }

    pub(crate) fn duplicate(
        &self,
        access: u32,
        inherit: bool,
        options: u32,
    ) -> io::Result<OwnedHandle> {
        let handle = self.as_raw_handle();

        // `Stdin`, `Stdout`, and `Stderr` can all hold null handles, such as
        // in a process with a detached console. `DuplicateHandle` would fail
        // if we passed it a null handle, but we can treat null as a valid
        // handle which doesn't do any I/O, and allow it to be duplicated.
        if handle.is_null() {
            return unsafe { Ok(OwnedHandle::from_raw_handle(handle)) };
        }

        let mut ret = ptr::null_mut();
        cvt(unsafe {
            let cur_proc = sys::c::GetCurrentProcess();
            sys::c::DuplicateHandle(
                cur_proc,
                handle,
                cur_proc,
                &mut ret,
                access,
                inherit as sys::c::BOOL,
                options,
            )
        })?;
        unsafe { Ok(OwnedHandle::from_raw_handle(ret)) }
    }
}

impl TryFrom<HandleOrInvalid> for OwnedHandle {
    type Error = InvalidHandleError;

    #[inline]
    fn try_from(handle_or_invalid: HandleOrInvalid) -> Result<Self, InvalidHandleError> {
        let owned_handle = handle_or_invalid.0;
        if owned_handle.handle == sys::c::INVALID_HANDLE_VALUE {
            // Don't call `CloseHandle`; it'd be harmless, except that it could
            // overwrite the `GetLastError` error.
            forget(owned_handle);

            Err(InvalidHandleError(()))
        } else {
            Ok(owned_handle)
        }
    }
}

/// This is the error type used by [`HandleOrNull`] when attempting to convert
/// into a handle, to indicate that the value is null.
// The empty field prevents constructing this, and allows extending it in the future.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullHandleError(());

impl fmt::Display for NullHandleError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        "A HandleOrNull could not be converted to a handle because it was null".fmt(fmt)
    }
}

impl crate::std::error::Error for NullHandleError {}

/// This is the error type used by [`HandleOrInvalid`] when attempting to
/// convert into a handle, to indicate that the value is
/// `INVALID_HANDLE_VALUE`.
// The empty field prevents constructing this, and allows extending it in the future.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidHandleError(());

impl fmt::Display for InvalidHandleError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        "A HandleOrInvalid could not be converted to a handle because it was INVALID_HANDLE_VALUE"
            .fmt(fmt)
    }
}

impl crate::std::error::Error for InvalidHandleError {}

impl AsRawHandle for BorrowedHandle<'_> {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

impl AsRawHandle for OwnedHandle {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

impl IntoRawHandle for OwnedHandle {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        let handle = self.handle;
        forget(self);
        handle
    }
}

impl FromRawHandle for OwnedHandle {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self { handle }
    }
}

impl HandleOrNull {
    /// Constructs a new instance of `Self` from the given `RawHandle` returned
    /// from a Windows API that uses null to indicate failure, such as
    /// `CreateThread`.
    ///
    /// Use `HandleOrInvalid` instead of `HandleOrNull` for APIs that
    /// use `INVALID_HANDLE_VALUE` to indicate failure.
    ///
    /// # Safety
    ///
    /// The passed `handle` value must either satisfy the safety requirements
    /// of [`FromRawHandle::from_raw_handle`], or be null. Note that not all
    /// Windows APIs use null for errors; see [here] for the full story.
    ///
    /// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
        #[inline]
    pub unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self(OwnedHandle::from_raw_handle(handle))
    }
}

impl HandleOrInvalid {
    /// Constructs a new instance of `Self` from the given `RawHandle` returned
    /// from a Windows API that uses `INVALID_HANDLE_VALUE` to indicate
    /// failure, such as `CreateFileW`.
    ///
    /// Use `HandleOrNull` instead of `HandleOrInvalid` for APIs that
    /// use null to indicate failure.
    ///
    /// # Safety
    ///
    /// The passed `handle` value must either satisfy the safety requirements
    /// of [`FromRawHandle::from_raw_handle`], or be
    /// `INVALID_HANDLE_VALUE` (-1). Note that not all Windows APIs use
    /// `INVALID_HANDLE_VALUE` for errors; see [here] for the full story.
    ///
    /// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
        #[inline]
    pub unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self(OwnedHandle::from_raw_handle(handle))
    }
}

impl Drop for OwnedHandle {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = sys::c::CloseHandle(self.handle);
        }
    }
}

impl fmt::Debug for BorrowedHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedHandle").field("handle", &self.handle).finish()
    }
}

impl fmt::Debug for OwnedHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedHandle").field("handle", &self.handle).finish()
    }
}

macro_rules! impl_is_terminal {
    ($($t:ty),*$(,)?) => {$(
                impl crate::std::sealed::Sealed for $t {}

                impl crate::std::io::IsTerminal for $t {
            #[inline]
            fn is_terminal(&self) -> bool {
                crate::std::sys::io::is_terminal(self)
            }
        }
    )*}
}

impl_is_terminal!(BorrowedHandle<'_>, OwnedHandle);

/// A trait to borrow the handle from an underlying object.
pub trait AsHandle {
    /// Borrows the handle.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use std::os::windows::io::{AsHandle, BorrowedHandle};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// let borrowed_handle: BorrowedHandle<'_> = f.as_handle();
    /// # Ok::<(), io::Error>(())
    /// ```
        fn as_handle(&self) -> BorrowedHandle<'_>;
}

impl<T: AsHandle> AsHandle for &T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        T::as_handle(self)
    }
}

impl<T: AsHandle> AsHandle for &mut T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        T::as_handle(self)
    }
}

/// This impl allows implementing traits that require `AsHandle` on Arc.
/// ```
/// # #[cfg(windows)] mod group_cfg {
/// # use std::os::windows::io::AsHandle;
/// use std::fs::File;
/// use std::sync::Arc;
///
/// trait MyTrait: AsHandle {}
/// impl MyTrait for Arc<File> {}
/// impl MyTrait for Box<File> {}
/// # }
/// ```
impl<T: AsHandle> AsHandle for crate::std::sync::Arc<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        (**self).as_handle()
    }
}

impl<T: AsHandle> AsHandle for crate::std::rc::Rc<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        (**self).as_handle()
    }
}

impl<T: AsHandle> AsHandle for Box<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        (**self).as_handle()
    }
}

impl AsHandle for BorrowedHandle<'_> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        *self
    }
}

impl AsHandle for OwnedHandle {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        // Safety: `OwnedHandle` and `BorrowedHandle` have the same validity
        // invariants, and the `BorrowedHandle` is bounded by the lifetime
        // of `&self`.
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl AsHandle for fs::File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.as_inner().as_handle()
    }
}

impl From<fs::File> for OwnedHandle {
    #[inline]
    fn from(file: fs::File) -> OwnedHandle {
        file.into_inner().into_inner().into_inner().into()
    }
}

impl From<OwnedHandle> for fs::File {
    #[inline]
    fn from(owned: OwnedHandle) -> Self {
        Self::from_inner(FromInner::from_inner(FromInner::from_inner(owned)))
    }
}

impl AsHandle for crate::std::io::Stdin {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl<'a> AsHandle for crate::std::io::StdinLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl AsHandle for crate::std::io::Stdout {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl<'a> AsHandle for crate::std::io::StdoutLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl AsHandle for crate::std::io::Stderr {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl<'a> AsHandle for crate::std::io::StderrLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl AsHandle for crate::std::process::ChildStdin {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl From<crate::std::process::ChildStdin> for OwnedHandle {
    #[inline]
    fn from(child_stdin: crate::std::process::ChildStdin) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(child_stdin.into_raw_handle()) }
    }
}

impl AsHandle for crate::std::process::ChildStdout {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl From<crate::std::process::ChildStdout> for OwnedHandle {
    #[inline]
    fn from(child_stdout: crate::std::process::ChildStdout) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(child_stdout.into_raw_handle()) }
    }
}

impl AsHandle for crate::std::process::ChildStderr {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl From<crate::std::process::ChildStderr> for OwnedHandle {
    #[inline]
    fn from(child_stderr: crate::std::process::ChildStderr) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(child_stderr.into_raw_handle()) }
    }
}

impl<T> AsHandle for crate::std::thread::JoinHandle<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl<T> From<crate::std::thread::JoinHandle<T>> for OwnedHandle {
    #[inline]
    fn from(join_handle: crate::std::thread::JoinHandle<T>) -> OwnedHandle {
        join_handle.into_inner().into_handle().into_inner()
    }
}
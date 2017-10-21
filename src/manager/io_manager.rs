//! Abstract manager used by `InputManager` and `OutputManager`

use libc;
use std::{mem, ptr};
use wayland_sys::server::WAYLAND_SERVER_HANDLE;
use wlroots_sys::wl_listener;

type NotifyFunc = unsafe extern "C" fn(*mut wl_listener, *mut libc::c_void);

#[repr(C)]
pub struct IOManager<T> {
    pub add_listener: wl_listener,
    pub remove_listener: wl_listener,
    pub manager: T
}

impl<T> IOManager<T> {
    pub fn new(manager: T, add_func: NotifyFunc, remove_func: NotifyFunc) -> Self {
        unsafe {
            // NOTE Rationale for zeroed memory:
            // * The list is initialized by Wayland, which doesn't "drop"
            // * The listener is written to without dropping any of the data
            let mut add_listener: wl_listener = mem::zeroed();
            let mut remove_listener: wl_listener = mem::zeroed();
            ffi_dispatch!(WAYLAND_SERVER_HANDLE,
                          wl_list_init,
                          &mut add_listener.link as *mut _ as _);
            ptr::write(&mut add_listener.notify, Some(add_func));
            ffi_dispatch!(WAYLAND_SERVER_HANDLE,
                          wl_list_init,
                          &mut remove_listener.link as *mut _ as _);
            ptr::write(&mut remove_listener.notify, Some(remove_func));
            IOManager {
                add_listener,
                remove_listener,
                manager
            }
        }
    }
}

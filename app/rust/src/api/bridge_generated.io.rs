use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_get_data(port_: i64) {
    wire_get_data_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_context() -> support::WireSyncReturn {
    wire_get_context_impl()
}

#[no_mangle]
pub extern "C" fn wire_get_fs(port_: i64, mlfs: wire_MultilangFs, ctx: wire_RwLockFaContext) {
    wire_get_fs_impl(port_, mlfs, ctx)
}

#[no_mangle]
pub extern "C" fn wire_transition(
    port_: i64,
    ctx: wire_RwLockFaContext,
    text: *mut wire_uint_8_list,
) {
    wire_transition_impl(port_, ctx, text)
}

#[no_mangle]
pub extern "C" fn wire_back(port_: i64, ctx: wire_RwLockFaContext) {
    wire_back_impl(port_, ctx)
}

#[no_mangle]
pub extern "C" fn wire_home(port_: i64, ctx: wire_RwLockFaContext) {
    wire_home_impl(port_, ctx)
}

#[no_mangle]
pub extern "C" fn wire_get_path(port_: i64, ctx: wire_RwLockFaContext) {
    wire_get_path_impl(port_, ctx)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MultilangFs() -> wire_MultilangFs {
    wire_MultilangFs::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RwLockFaContext() -> wire_RwLockFaContext {
    wire_RwLockFaContext::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_MultilangFs(ptr: *const c_void) {
    unsafe {
        Arc::<MultilangFs>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MultilangFs(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<MultilangFs>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockFaContext(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<FAContext>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockFaContext(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<FAContext>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<MultilangFs>> for wire_MultilangFs {
    fn wire2api(self) -> RustOpaque<MultilangFs> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<FAContext>>> for wire_RwLockFaContext {
    fn wire2api(self) -> RustOpaque<RwLock<FAContext>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MultilangFs {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockFaContext {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_MultilangFs {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockFaContext {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}

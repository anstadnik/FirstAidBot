use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_dynamic(port_: i64) {
    wire_dynamic_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_cached(port_: i64) {
    wire_cached_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_state(
    port_: i64,
    data: wire_RwLockData,
    ctx: *mut wire_StringList,
    lang: *mut wire_uint_8_list,
) {
    wire_get_state_impl(port_, data, ctx, lang)
}

#[no_mangle]
pub extern "C" fn wire_move_to_state(
    port_: i64,
    state: wire_RwLockState,
    text: *mut wire_uint_8_list,
    data: wire_RwLockData,
) {
    wire_move_to_state_impl(port_, state, text, data)
}

#[no_mangle]
pub extern "C" fn wire_back(port_: i64, state: wire_RwLockState) {
    wire_back_impl(port_, state)
}

#[no_mangle]
pub extern "C" fn wire_home(port_: i64, state: wire_RwLockState) {
    wire_home_impl(port_, state)
}

#[no_mangle]
pub extern "C" fn wire_depth(port_: i64, state: wire_RwLockState) {
    wire_depth_impl(port_, state)
}

#[no_mangle]
pub extern "C" fn wire_is_empty(port_: i64, state: wire_RwLockState) {
    wire_is_empty_impl(port_, state)
}

#[no_mangle]
pub extern "C" fn wire_context(port_: i64, state: wire_RwLockState) {
    wire_context_impl(port_, state)
}

#[no_mangle]
pub extern "C" fn wire_get_link(port_: i64, state: wire_RwLockState) {
    wire_get_link_impl(port_, state)
}

#[no_mangle]
pub extern "C" fn wire_get_message(port_: i64, state: wire_RwLockState) {
    wire_get_message_impl(port_, state)
}

#[no_mangle]
pub extern "C" fn wire_get_button_texts(port_: i64, state: wire_RwLockState) {
    wire_get_button_texts_impl(port_, state)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RwLockData() -> wire_RwLockData {
    wire_RwLockData::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RwLockState() -> wire_RwLockState {
    wire_RwLockState::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
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
pub extern "C" fn drop_opaque_RwLockData(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<Data>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockData(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<Data>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockState(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<State>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockState(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<State>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<RwLock<Data>>> for wire_RwLockData {
    fn wire2api(self) -> RustOpaque<RwLock<Data>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<State>>> for wire_RwLockState {
    fn wire2api(self) -> RustOpaque<RwLock<State>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
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
pub struct wire_RwLockData {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockState {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
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

impl NewWithNullPtr for wire_RwLockData {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockState {
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

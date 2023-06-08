use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_get_data(port_: MessagePort) {
    wire_get_data_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_context() -> support::WireSyncReturn {
    wire_get_context_impl()
}

#[wasm_bindgen]
pub fn wire_get_fs(port_: MessagePort, mlfs: JsValue, ctx: JsValue) {
    wire_get_fs_impl(port_, mlfs, ctx)
}

#[wasm_bindgen]
pub fn wire_transition(port_: MessagePort, ctx: JsValue, text: String) {
    wire_transition_impl(port_, ctx, text)
}

#[wasm_bindgen]
pub fn wire_back(port_: MessagePort, ctx: JsValue) {
    wire_back_impl(port_, ctx)
}

#[wasm_bindgen]
pub fn wire_home(port_: MessagePort, ctx: JsValue) {
    wire_home_impl(port_, ctx)
}

#[wasm_bindgen]
pub fn wire_get_path(port_: MessagePort, ctx: JsValue) {
    wire_get_path_impl(port_, ctx)
}

// Section: allocate functions

// Section: related functions

#[wasm_bindgen]
pub fn drop_opaque_MultilangFs(ptr: *const c_void) {
    unsafe {
        Arc::<MultilangFs>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MultilangFs(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<MultilangFs>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RwLockFaContext(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<FAContext>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockFaContext(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<FAContext>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<RustOpaque<MultilangFs>> for JsValue {
    fn wire2api(self) -> RustOpaque<MultilangFs> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<FAContext>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<FAContext>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}

use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_get_dynamic(port_: MessagePort) {
    wire_get_dynamic_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_cached(port_: MessagePort) {
    wire_get_cached_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_state(port_: MessagePort, data: JsValue, ctx: JsValue, lang: String) {
    wire_get_state_impl(port_, data, ctx, lang)
}

#[wasm_bindgen]
pub fn wire_move_to_state(port_: MessagePort, state: JsValue, text: String, data: JsValue) {
    wire_move_to_state_impl(port_, state, text, data)
}

#[wasm_bindgen]
pub fn wire_back(port_: MessagePort, state: JsValue) {
    wire_back_impl(port_, state)
}

#[wasm_bindgen]
pub fn wire_home(port_: MessagePort, state: JsValue) {
    wire_home_impl(port_, state)
}

#[wasm_bindgen]
pub fn wire_depth(state: JsValue) -> support::WireSyncReturn {
    wire_depth_impl(state)
}

#[wasm_bindgen]
pub fn wire_is_empty(state: JsValue) -> support::WireSyncReturn {
    wire_is_empty_impl(state)
}

#[wasm_bindgen]
pub fn wire_context(state: JsValue) -> support::WireSyncReturn {
    wire_context_impl(state)
}

#[wasm_bindgen]
pub fn wire_get_link(state: JsValue) -> support::WireSyncReturn {
    wire_get_link_impl(state)
}

#[wasm_bindgen]
pub fn wire_get_message(state: JsValue) -> support::WireSyncReturn {
    wire_get_message_impl(state)
}

#[wasm_bindgen]
pub fn wire_get_button_texts(state: JsValue) -> support::WireSyncReturn {
    wire_get_button_texts_impl(state)
}

// Section: allocate functions

// Section: related functions

#[wasm_bindgen]
pub fn drop_opaque_RwLockData(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<Data>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockData(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<Data>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RwLockState(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<State>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockState(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<State>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<RustOpaque<RwLock<Data>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<Data>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<State>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<State>> {
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

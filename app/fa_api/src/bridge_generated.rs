#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_get_dynamic_impl() -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get_dynamic",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || Ok(get_dynamic()),
    )
}
fn wire_get_cached_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_cached",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(get_cached()),
    )
}
fn wire_get_state_impl(
    data: impl Wire2Api<RustOpaque<RwLock<Data>>> + UnwindSafe,
    ctx: impl Wire2Api<Vec<String>> + UnwindSafe,
    lang: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get_state",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_data = data.wire2api();
            let api_ctx = ctx.wire2api();
            let api_lang = lang.wire2api();
            Ok(get_state(api_data, api_ctx, api_lang))
        },
    )
}
fn wire_move_to_state_impl(
    port_: MessagePort,
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
    text: impl Wire2Api<String> + UnwindSafe,
    data: impl Wire2Api<RustOpaque<RwLock<Data>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "move_to_state",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_state = state.wire2api();
            let api_text = text.wire2api();
            let api_data = data.wire2api();
            move |task_callback| move_to_state(api_state, api_text, api_data)
        },
    )
}
fn wire_back_impl(
    port_: MessagePort,
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "back",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_state = state.wire2api();
            move |task_callback| Ok(back(api_state))
        },
    )
}
fn wire_home_impl(
    port_: MessagePort,
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "home",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_state = state.wire2api();
            move |task_callback| Ok(home(api_state))
        },
    )
}
fn wire_depth_impl(
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "depth",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_state = state.wire2api();
            Ok(depth(api_state))
        },
    )
}
fn wire_is_empty_impl(
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "is_empty",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_state = state.wire2api();
            Ok(is_empty(api_state))
        },
    )
}
fn wire_context_impl(
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "context",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_state = state.wire2api();
            Ok(context(api_state))
        },
    )
}
fn wire_get_link_impl(
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get_link",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_state = state.wire2api();
            Ok(get_link(api_state))
        },
    )
}
fn wire_get_message_impl(
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get_message",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_state = state.wire2api();
            Ok(get_message(api_state))
        },
    )
}
fn wire_get_button_texts_impl(
    state: impl Wire2Api<RustOpaque<RwLock<State>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get_button_texts",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_state = state.wire2api();
            Ok(get_button_texts(api_state))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

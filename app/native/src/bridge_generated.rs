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
// Generated by `flutter_rust_bridge`@ 1.78.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_get_data_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_data",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(get_data()),
    )
}
fn wire_get_context_impl() -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get_context",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || Ok(get_context()),
    )
}
fn wire_get_fs_impl(
    port_: MessagePort,
    mlfs: impl Wire2Api<RustOpaque<MultilangFs>> + UnwindSafe,
    ctx: impl Wire2Api<RustOpaque<RwLock<FAContext>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_fs",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_mlfs = mlfs.wire2api();
            let api_ctx = ctx.wire2api();
            move |task_callback| Ok(get_fs(api_mlfs, api_ctx))
        },
    )
}
fn wire_transition_impl(
    port_: MessagePort,
    ctx: impl Wire2Api<RustOpaque<RwLock<FAContext>>> + UnwindSafe,
    text: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "transition",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ctx = ctx.wire2api();
            let api_text = text.wire2api();
            move |task_callback| Ok(transition(api_ctx, api_text))
        },
    )
}
fn wire_back_impl(
    port_: MessagePort,
    ctx: impl Wire2Api<RustOpaque<RwLock<FAContext>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "back",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ctx = ctx.wire2api();
            move |task_callback| Ok(back(api_ctx))
        },
    )
}
fn wire_home_impl(
    port_: MessagePort,
    ctx: impl Wire2Api<RustOpaque<RwLock<FAContext>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "home",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ctx = ctx.wire2api();
            move |task_callback| Ok(home(api_ctx))
        },
    )
}
fn wire_get_path_impl(
    port_: MessagePort,
    ctx: impl Wire2Api<RustOpaque<RwLock<FAContext>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_path",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_ctx = ctx.wire2api();
            move |task_callback| Ok(get_path(api_ctx))
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

impl support::IntoDart for FAState {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.link.into_dart(),
            self.message.into_dart(),
            self.next_states.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for FAState {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

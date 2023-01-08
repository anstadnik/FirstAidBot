// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.web.dart';
export 'bridge_generated.web.dart';
import 'package:meta/meta.dart';

class FaApiPlatform extends FlutterRustBridgeBase<FaApiWire>
    with FlutterRustBridgeSetupMixin {
  FaApiPlatform(FutureOr<WasmModule> dylib) : super(FaApiWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  Object api2wire_RwLockData(RwLockData raw) {
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RwLockState(RwLockState raw) {
    return raw.shareOrMove();
  }

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  List<String> api2wire_StringList(List<String> raw) {
    return raw;
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }
// Section: finalizer

  late final Finalizer<PlatformPointer> _RwLockDataFinalizer =
      Finalizer<PlatformPointer>(inner.drop_opaque_RwLockData);
  Finalizer<PlatformPointer> get RwLockDataFinalizer => _RwLockDataFinalizer;
  late final Finalizer<PlatformPointer> _RwLockStateFinalizer =
      Finalizer<PlatformPointer>(inner.drop_opaque_RwLockState);
  Finalizer<PlatformPointer> get RwLockStateFinalizer => _RwLockStateFinalizer;
}

// Section: WASM wire module

@JS('wasm_bindgen')
external FaApiWasmModule get wasmModule;

@JS()
@anonymous
class FaApiWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external FaApiWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* void */ wire_dynamic(NativePortType port_);

  external dynamic /* void */ wire_cached(NativePortType port_);

  external dynamic /* void */ wire_get_state(
      NativePortType port_, Object data, List<String> ctx, String lang);

  external dynamic /* void */ wire_move_to_state(
      NativePortType port_, Object state, String text, Object data);

  external dynamic /* void */ wire_back(NativePortType port_, Object state);

  external dynamic /* void */ wire_home(NativePortType port_, Object state);

  external dynamic /* void */ wire_depth(NativePortType port_, Object state);

  external dynamic /* void */ wire_is_empty(NativePortType port_, Object state);

  external dynamic /* void */ wire_context(NativePortType port_, Object state);

  external dynamic /* void */ wire_get_link(NativePortType port_, Object state);

  external dynamic /* void */ wire_get_message(
      NativePortType port_, Object state);

  external dynamic /* void */ wire_get_button_texts(
      NativePortType port_, Object state);

  external dynamic /*  */ drop_opaque_RwLockData(ptr);

  external int /* *const c_void */ share_opaque_RwLockData(ptr);

  external dynamic /*  */ drop_opaque_RwLockState(ptr);

  external int /* *const c_void */ share_opaque_RwLockState(ptr);
}

// Section: WASM wire connector

class FaApiWire extends FlutterRustBridgeWasmWireBase<FaApiWasmModule> {
  FaApiWire(FutureOr<WasmModule> module)
      : super(WasmModule.cast<FaApiWasmModule>(module));

  void wire_dynamic(NativePortType port_) => wasmModule.wire_dynamic(port_);

  void wire_cached(NativePortType port_) => wasmModule.wire_cached(port_);

  void wire_get_state(
          NativePortType port_, Object data, List<String> ctx, String lang) =>
      wasmModule.wire_get_state(port_, data, ctx, lang);

  void wire_move_to_state(
          NativePortType port_, Object state, String text, Object data) =>
      wasmModule.wire_move_to_state(port_, state, text, data);

  void wire_back(NativePortType port_, Object state) =>
      wasmModule.wire_back(port_, state);

  void wire_home(NativePortType port_, Object state) =>
      wasmModule.wire_home(port_, state);

  void wire_depth(NativePortType port_, Object state) =>
      wasmModule.wire_depth(port_, state);

  void wire_is_empty(NativePortType port_, Object state) =>
      wasmModule.wire_is_empty(port_, state);

  void wire_context(NativePortType port_, Object state) =>
      wasmModule.wire_context(port_, state);

  void wire_get_link(NativePortType port_, Object state) =>
      wasmModule.wire_get_link(port_, state);

  void wire_get_message(NativePortType port_, Object state) =>
      wasmModule.wire_get_message(port_, state);

  void wire_get_button_texts(NativePortType port_, Object state) =>
      wasmModule.wire_get_button_texts(port_, state);

  dynamic /*  */ drop_opaque_RwLockData(ptr) =>
      wasmModule.drop_opaque_RwLockData(ptr);

  int /* *const c_void */ share_opaque_RwLockData(ptr) =>
      wasmModule.share_opaque_RwLockData(ptr);

  dynamic /*  */ drop_opaque_RwLockState(ptr) =>
      wasmModule.drop_opaque_RwLockState(ptr);

  int /* *const c_void */ share_opaque_RwLockState(ptr) =>
      wasmModule.share_opaque_RwLockState(ptr);
}

// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'package:meta/meta.dart';
import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:meta/meta.dart';
import 'dart:ffi' as ffi;

class FaApiImpl implements FaApi {
  final FaApiPlatform _platform;
  factory FaApiImpl(ExternalLibrary dylib) =>
      FaApiImpl.raw(FaApiPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory FaApiImpl.wasm(FutureOr<WasmModule> module) =>
      FaApiImpl(module as ExternalLibrary);
  FaApiImpl.raw(this._platform);
  Future<RwLockData> getDynamic({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_get_dynamic(port_),
      parseSuccessData: _wire2api_RwLockData,
      constMeta: kGetDynamicConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetDynamicConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_dynamic",
        argNames: [],
      );

  Future<RwLockData> getCached({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_get_cached(port_),
      parseSuccessData: _wire2api_RwLockData,
      constMeta: kGetCachedConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetCachedConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_cached",
        argNames: [],
      );

  Future<RwLockState> getState(
      {required RwLockData data,
      required List<String> ctx,
      required String lang,
      dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockData(data);
    var arg1 = _platform.api2wire_StringList(ctx);
    var arg2 = _platform.api2wire_String(lang);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_get_state(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_RwLockState,
      constMeta: kGetStateConstMeta,
      argValues: [data, ctx, lang],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetStateConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_state",
        argNames: ["data", "ctx", "lang"],
      );

  Future<void> moveToState(
      {required RwLockState state,
      required String text,
      required RwLockData data,
      dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    var arg1 = _platform.api2wire_String(text);
    var arg2 = _platform.api2wire_RwLockData(data);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_move_to_state(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_unit,
      constMeta: kMoveToStateConstMeta,
      argValues: [state, text, data],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kMoveToStateConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "move_to_state",
        argNames: ["state", "text", "data"],
      );

  Future<void> back({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_back(port_, arg0),
      parseSuccessData: _wire2api_unit,
      constMeta: kBackConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kBackConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "back",
        argNames: ["state"],
      );

  Future<void> home({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_home(port_, arg0),
      parseSuccessData: _wire2api_unit,
      constMeta: kHomeConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kHomeConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "home",
        argNames: ["state"],
      );

  int depth({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_depth(arg0),
      parseSuccessData: _wire2api_usize,
      constMeta: kDepthConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kDepthConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "depth",
        argNames: ["state"],
      );

  bool isEmpty({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_is_empty(arg0),
      parseSuccessData: _wire2api_bool,
      constMeta: kIsEmptyConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kIsEmptyConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "is_empty",
        argNames: ["state"],
      );

  List<String> context({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_context(arg0),
      parseSuccessData: _wire2api_StringList,
      constMeta: kContextConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kContextConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "context",
        argNames: ["state"],
      );

  String? getLink({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_get_link(arg0),
      parseSuccessData: _wire2api_opt_String,
      constMeta: kGetLinkConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetLinkConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_link",
        argNames: ["state"],
      );

  String getMessage({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_get_message(arg0),
      parseSuccessData: _wire2api_String,
      constMeta: kGetMessageConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetMessageConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_message",
        argNames: ["state"],
      );

  List<String> getButtonTexts({required RwLockState state, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockState(state);
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_get_button_texts(arg0),
      parseSuccessData: _wire2api_StringList,
      constMeta: kGetButtonTextsConstMeta,
      argValues: [state],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetButtonTextsConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_button_texts",
        argNames: ["state"],
      );

  DropFnType get dropOpaqueRwLockData => _platform.inner.drop_opaque_RwLockData;
  ShareFnType get shareOpaqueRwLockData =>
      _platform.inner.share_opaque_RwLockData;
  OpaqueTypeFinalizer get RwLockDataFinalizer => _platform.RwLockDataFinalizer;

  DropFnType get dropOpaqueRwLockState =>
      _platform.inner.drop_opaque_RwLockState;
  ShareFnType get shareOpaqueRwLockState =>
      _platform.inner.share_opaque_RwLockState;
  OpaqueTypeFinalizer get RwLockStateFinalizer =>
      _platform.RwLockStateFinalizer;

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  RwLockData _wire2api_RwLockData(dynamic raw) {
    return RwLockData.fromRaw(raw[0], raw[1], this);
  }

  RwLockState _wire2api_RwLockState(dynamic raw) {
    return RwLockState.fromRaw(raw[0], raw[1], this);
  }

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  List<String> _wire2api_StringList(dynamic raw) {
    return (raw as List<dynamic>).cast<String>();
  }

  bool _wire2api_bool(dynamic raw) {
    return raw as bool;
  }

  String? _wire2api_opt_String(dynamic raw) {
    return raw == null ? null : _wire2api_String(raw);
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }

  void _wire2api_unit(dynamic raw) {
    return;
  }

  int _wire2api_usize(dynamic raw) {
    return castInt(raw);
  }
}

// Section: api2wire

@protected
int api2wire_u8(int raw) {
  return raw;
}

// Section: finalizer

class FaApiPlatform extends FlutterRustBridgeBase<FaApiWire> {
  FaApiPlatform(ffi.DynamicLibrary dylib) : super(FaApiWire(dylib));

// Section: api2wire

  @protected
  wire_RwLockData api2wire_RwLockData(RwLockData raw) {
    final ptr = inner.new_RwLockData();
    _api_fill_to_wire_RwLockData(raw, ptr);
    return ptr;
  }

  @protected
  wire_RwLockState api2wire_RwLockState(RwLockState raw) {
    final ptr = inner.new_RwLockState();
    _api_fill_to_wire_RwLockState(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_StringList> api2wire_StringList(List<String> raw) {
    final ans = inner.new_StringList_0(raw.length);
    for (var i = 0; i < raw.length; i++) {
      ans.ref.ptr[i] = api2wire_String(raw[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

  late final OpaqueTypeFinalizer _RwLockDataFinalizer =
      OpaqueTypeFinalizer(inner._drop_opaque_RwLockDataPtr);
  OpaqueTypeFinalizer get RwLockDataFinalizer => _RwLockDataFinalizer;
  late final OpaqueTypeFinalizer _RwLockStateFinalizer =
      OpaqueTypeFinalizer(inner._drop_opaque_RwLockStatePtr);
  OpaqueTypeFinalizer get RwLockStateFinalizer => _RwLockStateFinalizer;
// Section: api_fill_to_wire

  void _api_fill_to_wire_RwLockData(
      RwLockData apiObj, wire_RwLockData wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RwLockState(
      RwLockState apiObj, wire_RwLockState wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class FaApiWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  FaApiWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  FaApiWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>(
          'get_dart_object');
  late final _get_dart_object =
      _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>(
          'drop_dart_object');
  late final _drop_dart_object =
      _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>(
          'new_dart_opaque');
  late final _new_dart_opaque =
      _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_get_dynamic(
    int port_,
  ) {
    return _wire_get_dynamic(
      port_,
    );
  }

  late final _wire_get_dynamicPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_dynamic');
  late final _wire_get_dynamic =
      _wire_get_dynamicPtr.asFunction<void Function(int)>();

  void wire_get_cached(
    int port_,
  ) {
    return _wire_get_cached(
      port_,
    );
  }

  late final _wire_get_cachedPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_cached');
  late final _wire_get_cached =
      _wire_get_cachedPtr.asFunction<void Function(int)>();

  void wire_get_state(
    int port_,
    wire_RwLockData data,
    ffi.Pointer<wire_StringList> ctx,
    ffi.Pointer<wire_uint_8_list> lang,
  ) {
    return _wire_get_state(
      port_,
      data,
      ctx,
      lang,
    );
  }

  late final _wire_get_statePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              wire_RwLockData,
              ffi.Pointer<wire_StringList>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_get_state');
  late final _wire_get_state = _wire_get_statePtr.asFunction<
      void Function(int, wire_RwLockData, ffi.Pointer<wire_StringList>,
          ffi.Pointer<wire_uint_8_list>)>();

  void wire_move_to_state(
    int port_,
    wire_RwLockState state,
    ffi.Pointer<wire_uint_8_list> text,
    wire_RwLockData data,
  ) {
    return _wire_move_to_state(
      port_,
      state,
      text,
      data,
    );
  }

  late final _wire_move_to_statePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              wire_RwLockState,
              ffi.Pointer<wire_uint_8_list>,
              wire_RwLockData)>>('wire_move_to_state');
  late final _wire_move_to_state = _wire_move_to_statePtr.asFunction<
      void Function(int, wire_RwLockState, ffi.Pointer<wire_uint_8_list>,
          wire_RwLockData)>();

  void wire_back(
    int port_,
    wire_RwLockState state,
  ) {
    return _wire_back(
      port_,
      state,
    );
  }

  late final _wire_backPtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_RwLockState)>>(
      'wire_back');
  late final _wire_back =
      _wire_backPtr.asFunction<void Function(int, wire_RwLockState)>();

  void wire_home(
    int port_,
    wire_RwLockState state,
  ) {
    return _wire_home(
      port_,
      state,
    );
  }

  late final _wire_homePtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_RwLockState)>>(
      'wire_home');
  late final _wire_home =
      _wire_homePtr.asFunction<void Function(int, wire_RwLockState)>();

  WireSyncReturn wire_depth(
    wire_RwLockState state,
  ) {
    return _wire_depth(
      state,
    );
  }

  late final _wire_depthPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockState)>>(
          'wire_depth');
  late final _wire_depth =
      _wire_depthPtr.asFunction<WireSyncReturn Function(wire_RwLockState)>();

  WireSyncReturn wire_is_empty(
    wire_RwLockState state,
  ) {
    return _wire_is_empty(
      state,
    );
  }

  late final _wire_is_emptyPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockState)>>(
          'wire_is_empty');
  late final _wire_is_empty =
      _wire_is_emptyPtr.asFunction<WireSyncReturn Function(wire_RwLockState)>();

  WireSyncReturn wire_context(
    wire_RwLockState state,
  ) {
    return _wire_context(
      state,
    );
  }

  late final _wire_contextPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockState)>>(
          'wire_context');
  late final _wire_context =
      _wire_contextPtr.asFunction<WireSyncReturn Function(wire_RwLockState)>();

  WireSyncReturn wire_get_link(
    wire_RwLockState state,
  ) {
    return _wire_get_link(
      state,
    );
  }

  late final _wire_get_linkPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockState)>>(
          'wire_get_link');
  late final _wire_get_link =
      _wire_get_linkPtr.asFunction<WireSyncReturn Function(wire_RwLockState)>();

  WireSyncReturn wire_get_message(
    wire_RwLockState state,
  ) {
    return _wire_get_message(
      state,
    );
  }

  late final _wire_get_messagePtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockState)>>(
          'wire_get_message');
  late final _wire_get_message = _wire_get_messagePtr
      .asFunction<WireSyncReturn Function(wire_RwLockState)>();

  WireSyncReturn wire_get_button_texts(
    wire_RwLockState state,
  ) {
    return _wire_get_button_texts(
      state,
    );
  }

  late final _wire_get_button_textsPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockState)>>(
          'wire_get_button_texts');
  late final _wire_get_button_texts = _wire_get_button_textsPtr
      .asFunction<WireSyncReturn Function(wire_RwLockState)>();

  wire_RwLockData new_RwLockData() {
    return _new_RwLockData();
  }

  late final _new_RwLockDataPtr =
      _lookup<ffi.NativeFunction<wire_RwLockData Function()>>('new_RwLockData');
  late final _new_RwLockData =
      _new_RwLockDataPtr.asFunction<wire_RwLockData Function()>();

  wire_RwLockState new_RwLockState() {
    return _new_RwLockState();
  }

  late final _new_RwLockStatePtr =
      _lookup<ffi.NativeFunction<wire_RwLockState Function()>>(
          'new_RwLockState');
  late final _new_RwLockState =
      _new_RwLockStatePtr.asFunction<wire_RwLockState Function()>();

  ffi.Pointer<wire_StringList> new_StringList_0(
    int len,
  ) {
    return _new_StringList_0(
      len,
    );
  }

  late final _new_StringList_0Ptr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_StringList> Function(ffi.Int32)>>(
      'new_StringList_0');
  late final _new_StringList_0 = _new_StringList_0Ptr
      .asFunction<ffi.Pointer<wire_StringList> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void drop_opaque_RwLockData(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RwLockData(
      ptr,
    );
  }

  late final _drop_opaque_RwLockDataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RwLockData');
  late final _drop_opaque_RwLockData = _drop_opaque_RwLockDataPtr
      .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RwLockData(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RwLockData(
      ptr,
    );
  }

  late final _share_opaque_RwLockDataPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RwLockData');
  late final _share_opaque_RwLockData = _share_opaque_RwLockDataPtr
      .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RwLockState(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RwLockState(
      ptr,
    );
  }

  late final _drop_opaque_RwLockStatePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RwLockState');
  late final _drop_opaque_RwLockState = _drop_opaque_RwLockStatePtr
      .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RwLockState(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RwLockState(
      ptr,
    );
  }

  late final _share_opaque_RwLockStatePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RwLockState');
  late final _share_opaque_RwLockState = _share_opaque_RwLockStatePtr
      .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>(
          'free_WireSyncReturn');
  late final _free_WireSyncReturn =
      _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

class _Dart_Handle extends ffi.Opaque {}

class wire_RwLockData extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

class wire_StringList extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<wire_uint_8_list>> ptr;

  @ffi.Int32()
  external int len;
}

class wire_RwLockState extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;

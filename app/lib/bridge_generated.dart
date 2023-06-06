// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.77.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.io.dart'
    if (dart.library.html) 'bridge_generated.web.dart';

class NativeImpl implements Native {
  final NativePlatform _platform;
  factory NativeImpl(ExternalLibrary dylib) =>
      NativeImpl.raw(NativePlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory NativeImpl.wasm(FutureOr<WasmModule> module) =>
      NativeImpl(module as ExternalLibrary);
  NativeImpl.raw(this._platform);
  Future<MultilangFs> getData({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_get_data(port_),
      parseSuccessData: _wire2api_MultilangFs,
      constMeta: kGetDataConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetDataConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_data",
        argNames: [],
      );

  RwLockFaContext getContext({dynamic hint}) {
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_get_context(),
      parseSuccessData: _wire2api_RwLockFaContext,
      constMeta: kGetContextConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetContextConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_context",
        argNames: [],
      );

  Future<FAState?> getFs(
      {required MultilangFs mlfs, required RwLockFaContext ctx, dynamic hint}) {
    var arg0 = _platform.api2wire_MultilangFs(mlfs);
    var arg1 = _platform.api2wire_RwLockFaContext(ctx);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_get_fs(port_, arg0, arg1),
      parseSuccessData: _wire2api_opt_box_autoadd_fa_state,
      constMeta: kGetFsConstMeta,
      argValues: [mlfs, ctx],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kGetFsConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_fs",
        argNames: ["mlfs", "ctx"],
      );

  Future<void> transition(
      {required RwLockFaContext ctx, required String text, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockFaContext(ctx);
    var arg1 = _platform.api2wire_String(text);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_transition(port_, arg0, arg1),
      parseSuccessData: _wire2api_unit,
      constMeta: kTransitionConstMeta,
      argValues: [ctx, text],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kTransitionConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "transition",
        argNames: ["ctx", "text"],
      );

  Future<void> back({required RwLockFaContext ctx, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockFaContext(ctx);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_back(port_, arg0),
      parseSuccessData: _wire2api_unit,
      constMeta: kBackConstMeta,
      argValues: [ctx],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kBackConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "back",
        argNames: ["ctx"],
      );

  Future<void> home({required RwLockFaContext ctx, dynamic hint}) {
    var arg0 = _platform.api2wire_RwLockFaContext(ctx);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_home(port_, arg0),
      parseSuccessData: _wire2api_unit,
      constMeta: kHomeConstMeta,
      argValues: [ctx],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kHomeConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "home",
        argNames: ["ctx"],
      );

  DropFnType get dropOpaqueMultilangFs =>
      _platform.inner.drop_opaque_MultilangFs;
  ShareFnType get shareOpaqueMultilangFs =>
      _platform.inner.share_opaque_MultilangFs;
  OpaqueTypeFinalizer get MultilangFsFinalizer =>
      _platform.MultilangFsFinalizer;

  DropFnType get dropOpaqueRwLockFaContext =>
      _platform.inner.drop_opaque_RwLockFaContext;
  ShareFnType get shareOpaqueRwLockFaContext =>
      _platform.inner.share_opaque_RwLockFaContext;
  OpaqueTypeFinalizer get RwLockFaContextFinalizer =>
      _platform.RwLockFaContextFinalizer;

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  MultilangFs _wire2api_MultilangFs(dynamic raw) {
    return MultilangFs.fromRaw(raw[0], raw[1], this);
  }

  RwLockFaContext _wire2api_RwLockFaContext(dynamic raw) {
    return RwLockFaContext.fromRaw(raw[0], raw[1], this);
  }

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  List<String> _wire2api_StringList(dynamic raw) {
    return (raw as List<dynamic>).cast<String>();
  }

  FAState _wire2api_box_autoadd_fa_state(dynamic raw) {
    return _wire2api_fa_state(raw);
  }

  FAState _wire2api_fa_state(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return FAState(
      link: _wire2api_opt_String(arr[0]),
      message: _wire2api_String(arr[1]),
      nextStates: _wire2api_StringList(arr[2]),
    );
  }

  String? _wire2api_opt_String(dynamic raw) {
    return raw == null ? null : _wire2api_String(raw);
  }

  FAState? _wire2api_opt_box_autoadd_fa_state(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_fa_state(raw);
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
}

// Section: api2wire

@protected
int api2wire_u8(int raw) {
  return raw;
}

// Section: finalizer

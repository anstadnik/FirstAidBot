// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.78.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'dart:ffi' as ffi;

class NativePlatform extends FlutterRustBridgeBase<NativeWire> {
  NativePlatform(ffi.DynamicLibrary dylib) : super(NativeWire(dylib));

// Section: api2wire

  @protected
  wire_MultilangFs api2wire_MultilangFs(MultilangFs raw) {
    final ptr = inner.new_MultilangFs();
    _api_fill_to_wire_MultilangFs(raw, ptr);
    return ptr;
  }

  @protected
  wire_RwLockFaContext api2wire_RwLockFaContext(RwLockFaContext raw) {
    final ptr = inner.new_RwLockFaContext();
    _api_fill_to_wire_RwLockFaContext(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

  late final OpaqueTypeFinalizer _MultilangFsFinalizer =
      OpaqueTypeFinalizer(inner._drop_opaque_MultilangFsPtr);
  OpaqueTypeFinalizer get MultilangFsFinalizer => _MultilangFsFinalizer;
  late final OpaqueTypeFinalizer _RwLockFaContextFinalizer =
      OpaqueTypeFinalizer(inner._drop_opaque_RwLockFaContextPtr);
  OpaqueTypeFinalizer get RwLockFaContextFinalizer => _RwLockFaContextFinalizer;
// Section: api_fill_to_wire

  void _api_fill_to_wire_MultilangFs(
      MultilangFs apiObj, wire_MultilangFs wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RwLockFaContext(
      RwLockFaContext apiObj, wire_RwLockFaContext wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class NativeWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  NativeWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  NativeWire.fromLookup(
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

  void wire_get_data(
    int port_,
  ) {
    return _wire_get_data(
      port_,
    );
  }

  late final _wire_get_dataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_data');
  late final _wire_get_data =
      _wire_get_dataPtr.asFunction<void Function(int)>();

  WireSyncReturn wire_get_context() {
    return _wire_get_context();
  }

  late final _wire_get_contextPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_get_context');
  late final _wire_get_context =
      _wire_get_contextPtr.asFunction<WireSyncReturn Function()>();

  void wire_get_fs(
    int port_,
    wire_MultilangFs mlfs,
    wire_RwLockFaContext ctx,
  ) {
    return _wire_get_fs(
      port_,
      mlfs,
      ctx,
    );
  }

  late final _wire_get_fsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, wire_MultilangFs,
              wire_RwLockFaContext)>>('wire_get_fs');
  late final _wire_get_fs = _wire_get_fsPtr
      .asFunction<void Function(int, wire_MultilangFs, wire_RwLockFaContext)>();

  void wire_transition(
    int port_,
    wire_RwLockFaContext ctx,
    ffi.Pointer<wire_uint_8_list> text,
  ) {
    return _wire_transition(
      port_,
      ctx,
      text,
    );
  }

  late final _wire_transitionPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, wire_RwLockFaContext,
              ffi.Pointer<wire_uint_8_list>)>>('wire_transition');
  late final _wire_transition = _wire_transitionPtr.asFunction<
      void Function(
          int, wire_RwLockFaContext, ffi.Pointer<wire_uint_8_list>)>();

  void wire_back(
    int port_,
    wire_RwLockFaContext ctx,
  ) {
    return _wire_back(
      port_,
      ctx,
    );
  }

  late final _wire_backPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, wire_RwLockFaContext)>>('wire_back');
  late final _wire_back =
      _wire_backPtr.asFunction<void Function(int, wire_RwLockFaContext)>();

  void wire_home(
    int port_,
    wire_RwLockFaContext ctx,
  ) {
    return _wire_home(
      port_,
      ctx,
    );
  }

  late final _wire_homePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, wire_RwLockFaContext)>>('wire_home');
  late final _wire_home =
      _wire_homePtr.asFunction<void Function(int, wire_RwLockFaContext)>();

  void wire_get_path(
    int port_,
    wire_RwLockFaContext ctx,
  ) {
    return _wire_get_path(
      port_,
      ctx,
    );
  }

  late final _wire_get_pathPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, wire_RwLockFaContext)>>('wire_get_path');
  late final _wire_get_path =
      _wire_get_pathPtr.asFunction<void Function(int, wire_RwLockFaContext)>();

  wire_MultilangFs new_MultilangFs() {
    return _new_MultilangFs();
  }

  late final _new_MultilangFsPtr =
      _lookup<ffi.NativeFunction<wire_MultilangFs Function()>>(
          'new_MultilangFs');
  late final _new_MultilangFs =
      _new_MultilangFsPtr.asFunction<wire_MultilangFs Function()>();

  wire_RwLockFaContext new_RwLockFaContext() {
    return _new_RwLockFaContext();
  }

  late final _new_RwLockFaContextPtr =
      _lookup<ffi.NativeFunction<wire_RwLockFaContext Function()>>(
          'new_RwLockFaContext');
  late final _new_RwLockFaContext =
      _new_RwLockFaContextPtr.asFunction<wire_RwLockFaContext Function()>();

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

  void drop_opaque_MultilangFs(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_MultilangFs(
      ptr,
    );
  }

  late final _drop_opaque_MultilangFsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_MultilangFs');
  late final _drop_opaque_MultilangFs = _drop_opaque_MultilangFsPtr
      .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_MultilangFs(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_MultilangFs(
      ptr,
    );
  }

  late final _share_opaque_MultilangFsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_MultilangFs');
  late final _share_opaque_MultilangFs = _share_opaque_MultilangFsPtr
      .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RwLockFaContext(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RwLockFaContext(
      ptr,
    );
  }

  late final _drop_opaque_RwLockFaContextPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RwLockFaContext');
  late final _drop_opaque_RwLockFaContext = _drop_opaque_RwLockFaContextPtr
      .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RwLockFaContext(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RwLockFaContext(
      ptr,
    );
  }

  late final _share_opaque_RwLockFaContextPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RwLockFaContext');
  late final _share_opaque_RwLockFaContext = _share_opaque_RwLockFaContextPtr
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

final class _Dart_Handle extends ffi.Opaque {}

final class wire_MultilangFs extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_RwLockFaContext extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<
        ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;

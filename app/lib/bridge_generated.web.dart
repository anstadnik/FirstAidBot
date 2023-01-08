// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import "bridge_definitions.web.dart";
import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.web.io.dart'
    if (dart.library.html) 'bridge_generated.web.web.dart';

import 'package:meta/meta.dart';

class FirstAidBotCoreImpl implements FirstAidBotCore {
  final FirstAidBotCorePlatform _platform;
  factory FirstAidBotCoreImpl(ExternalLibrary dylib) =>
      FirstAidBotCoreImpl.raw(FirstAidBotCorePlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory FirstAidBotCoreImpl.wasm(FutureOr<WasmModule> module) =>
      FirstAidBotCoreImpl(module as ExternalLibrary);
  FirstAidBotCoreImpl.raw(this._platform);
  Future<String> helloWorld({dynamic hint}) {
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_hello_world(port_),
      parseSuccessData: _wire2api_String,
      constMeta: kHelloWorldConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kHelloWorldConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "hello_world",
        argNames: [],
      );

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }
}

// Section: api2wire

// Section: finalizer

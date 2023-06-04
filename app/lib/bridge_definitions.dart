// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;

abstract class FaApi {
  RwLockData getDynamic({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetDynamicConstMeta;

  Future<RwLockData> getCached({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetCachedConstMeta;

  RwLockState getState(
      {required RwLockData data,
      required List<String> ctx,
      required String lang,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetStateConstMeta;

  Future<void> moveToState(
      {required RwLockState state,
      required String text,
      required RwLockData data,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kMoveToStateConstMeta;

  Future<void> back({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kBackConstMeta;

  Future<void> home({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kHomeConstMeta;

  int depth({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kDepthConstMeta;

  bool isEmpty({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kIsEmptyConstMeta;

  List<String> context({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kContextConstMeta;

  String? getLink({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetLinkConstMeta;

  String getMessage({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetMessageConstMeta;

  List<String> getButtonTexts({required RwLockState state, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetButtonTextsConstMeta;

  DropFnType get dropOpaqueRwLockData;
  ShareFnType get shareOpaqueRwLockData;
  OpaqueTypeFinalizer get RwLockDataFinalizer;

  DropFnType get dropOpaqueRwLockState;
  ShareFnType get shareOpaqueRwLockState;
  OpaqueTypeFinalizer get RwLockStateFinalizer;
}

@sealed
class RwLockData extends FrbOpaque {
  final FaApi bridge;
  RwLockData.fromRaw(int ptr, int size, this.bridge) : super.unsafe(ptr, size);
  @override
  DropFnType get dropFn => bridge.dropOpaqueRwLockData;

  @override
  ShareFnType get shareFn => bridge.shareOpaqueRwLockData;

  @override
  OpaqueTypeFinalizer get staticFinalizer => bridge.RwLockDataFinalizer;
}

@sealed
class RwLockState extends FrbOpaque {
  final FaApi bridge;
  RwLockState.fromRaw(int ptr, int size, this.bridge) : super.unsafe(ptr, size);
  @override
  DropFnType get dropFn => bridge.dropOpaqueRwLockState;

  @override
  ShareFnType get shareFn => bridge.shareOpaqueRwLockState;

  @override
  OpaqueTypeFinalizer get staticFinalizer => bridge.RwLockStateFinalizer;
}
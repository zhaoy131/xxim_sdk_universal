// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.62.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';

class XximSdkPlatform extends FlutterRustBridgeBase<XximSdkWire> with FlutterRustBridgeSetupMixin {
  XximSdkPlatform(FutureOr<WasmModule> dylib) : super(XximSdkWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  Object api2wire_i64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }
// Section: finalizer
}

// Section: WASM wire module

@JS('wasm_bindgen')
external XximSdkWasmModule get wasmModule;

@JS()
@anonymous
class XximSdkWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external XximSdkWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* void */ wire_new_instance(NativePortType port_);

  external dynamic /* void */ wire_destroy_instance(NativePortType port_, String instance_id);

  external dynamic /* void */ wire_init(NativePortType port_, String instance_id, String params);

  external dynamic /* void */ wire_set_login_info(NativePortType port_, String instance_id, String params);

  external dynamic /* void */ wire_unset_login_info(NativePortType port_, String instance_id);

  external dynamic /* void */ wire_context_with_timeout(NativePortType port_, String instance_id, Object timeout_mills);

  external dynamic /* void */ wire_user_register_api(NativePortType port_, String instance_id, String ctx, Uint8List protobuf);
}

// Section: WASM wire connector

class XximSdkWire extends FlutterRustBridgeWasmWireBase<XximSdkWasmModule> {
  XximSdkWire(FutureOr<WasmModule> module) : super(WasmModule.cast<XximSdkWasmModule>(module));

  void wire_new_instance(NativePortType port_) => wasmModule.wire_new_instance(port_);

  void wire_destroy_instance(NativePortType port_, String instance_id) => wasmModule.wire_destroy_instance(port_, instance_id);

  void wire_init(NativePortType port_, String instance_id, String params) => wasmModule.wire_init(port_, instance_id, params);

  void wire_set_login_info(NativePortType port_, String instance_id, String params) => wasmModule.wire_set_login_info(port_, instance_id, params);

  void wire_unset_login_info(NativePortType port_, String instance_id) => wasmModule.wire_unset_login_info(port_, instance_id);

  void wire_context_with_timeout(NativePortType port_, String instance_id, Object timeout_mills) => wasmModule.wire_context_with_timeout(port_, instance_id, timeout_mills);

  void wire_user_register_api(NativePortType port_, String instance_id, String ctx, Uint8List protobuf) => wasmModule.wire_user_register_api(port_, instance_id, ctx, protobuf);
}

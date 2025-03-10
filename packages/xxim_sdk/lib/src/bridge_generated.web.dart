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
  String? api2wire_opt_String(String? raw) {
    return raw == null ? null : api2wire_String(raw);
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
  external dynamic /* void */ wire_new_instance(NativePortType port_, String instance_id);

  external dynamic /* void */ wire_destroy_instance(NativePortType port_, String instance_id);

  external dynamic /* void */ wire_init_instance(NativePortType port_, String instance_id, String host, int port, bool ssl, String? app_id, String? install_id, int platform, String device_model, String os_version, int language, int request_timeout_millisecond, String db_dir, String? custom_header, int keep_alive_second, int log_level);

  external dynamic /* void */ wire_preset_stream(NativePortType port_, String instance_id);

  external dynamic /* void */ wire_wait_stream_ready(NativePortType port_, String instance_id);

  external dynamic /* void */ wire_set_login_info(NativePortType port_, String instance_id, String token, String user_id);

  external dynamic /* void */ wire_unset_login_info(NativePortType port_, String instance_id);

  external dynamic /* void */ wire_user_register(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_user_access_token(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_create_robot(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_refresh_user_access_token(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_revoke_user_access_token(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_friend_apply(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_list_friend_apply(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_friend_apply_handle(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_group_create(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_message_send(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_message_batch_send(NativePortType port_, String instance_id, Uint8List protobuf);

  external dynamic /* void */ wire_list_notice(NativePortType port_, String instance_id, Uint8List protobuf);
}

// Section: WASM wire connector

class XximSdkWire extends FlutterRustBridgeWasmWireBase<XximSdkWasmModule> {
  XximSdkWire(FutureOr<WasmModule> module) : super(WasmModule.cast<XximSdkWasmModule>(module));

  void wire_new_instance(NativePortType port_, String instance_id) => wasmModule.wire_new_instance(port_, instance_id);

  void wire_destroy_instance(NativePortType port_, String instance_id) => wasmModule.wire_destroy_instance(port_, instance_id);

  void wire_init_instance(NativePortType port_, String instance_id, String host, int port, bool ssl, String? app_id, String? install_id, int platform, String device_model, String os_version, int language, int request_timeout_millisecond, String db_dir, String? custom_header, int keep_alive_second, int log_level) => wasmModule.wire_init_instance(port_, instance_id, host, port, ssl, app_id, install_id, platform, device_model, os_version, language, request_timeout_millisecond, db_dir, custom_header, keep_alive_second, log_level);

  void wire_preset_stream(NativePortType port_, String instance_id) => wasmModule.wire_preset_stream(port_, instance_id);

  void wire_wait_stream_ready(NativePortType port_, String instance_id) => wasmModule.wire_wait_stream_ready(port_, instance_id);

  void wire_set_login_info(NativePortType port_, String instance_id, String token, String user_id) => wasmModule.wire_set_login_info(port_, instance_id, token, user_id);

  void wire_unset_login_info(NativePortType port_, String instance_id) => wasmModule.wire_unset_login_info(port_, instance_id);

  void wire_user_register(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_user_register(port_, instance_id, protobuf);

  void wire_user_access_token(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_user_access_token(port_, instance_id, protobuf);

  void wire_create_robot(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_create_robot(port_, instance_id, protobuf);

  void wire_refresh_user_access_token(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_refresh_user_access_token(port_, instance_id, protobuf);

  void wire_revoke_user_access_token(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_revoke_user_access_token(port_, instance_id, protobuf);

  void wire_friend_apply(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_friend_apply(port_, instance_id, protobuf);

  void wire_list_friend_apply(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_list_friend_apply(port_, instance_id, protobuf);

  void wire_friend_apply_handle(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_friend_apply_handle(port_, instance_id, protobuf);

  void wire_group_create(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_group_create(port_, instance_id, protobuf);

  void wire_message_send(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_message_send(port_, instance_id, protobuf);

  void wire_message_batch_send(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_message_batch_send(port_, instance_id, protobuf);

  void wire_list_notice(NativePortType port_, String instance_id, Uint8List protobuf) => wasmModule.wire_list_notice(port_, instance_id, protobuf);
}

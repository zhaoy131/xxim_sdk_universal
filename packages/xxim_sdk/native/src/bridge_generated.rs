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
// Generated by `flutter_rust_bridge`@ 1.62.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_new_instance_impl(port_: MessagePort, instance_id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_instance",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            move |task_callback| Ok(new_instance(api_instance_id))
        },
    )
}
fn wire_destroy_instance_impl(port_: MessagePort, instance_id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "destroy_instance",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            move |task_callback| Ok(destroy_instance(api_instance_id))
        },
    )
}
fn wire_init_instance_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    host: impl Wire2Api<String> + UnwindSafe,
    port: impl Wire2Api<u16> + UnwindSafe,
    ssl: impl Wire2Api<bool> + UnwindSafe,
    app_id: impl Wire2Api<Option<String>> + UnwindSafe,
    install_id: impl Wire2Api<Option<String>> + UnwindSafe,
    platform: impl Wire2Api<i32> + UnwindSafe,
    device_model: impl Wire2Api<String> + UnwindSafe,
    os_version: impl Wire2Api<String> + UnwindSafe,
    language: impl Wire2Api<i32> + UnwindSafe,
    request_timeout_millisecond: impl Wire2Api<i32> + UnwindSafe,
    db_dir: impl Wire2Api<String> + UnwindSafe,
    custom_header: impl Wire2Api<Option<String>> + UnwindSafe,
    keep_alive_second: impl Wire2Api<i32> + UnwindSafe,
    log_level: impl Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init_instance",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_host = host.wire2api();
            let api_port = port.wire2api();
            let api_ssl = ssl.wire2api();
            let api_app_id = app_id.wire2api();
            let api_install_id = install_id.wire2api();
            let api_platform = platform.wire2api();
            let api_device_model = device_model.wire2api();
            let api_os_version = os_version.wire2api();
            let api_language = language.wire2api();
            let api_request_timeout_millisecond = request_timeout_millisecond.wire2api();
            let api_db_dir = db_dir.wire2api();
            let api_custom_header = custom_header.wire2api();
            let api_keep_alive_second = keep_alive_second.wire2api();
            let api_log_level = log_level.wire2api();
            move |task_callback| {
                Ok(init_instance(
                    api_instance_id,
                    api_host,
                    api_port,
                    api_ssl,
                    api_app_id,
                    api_install_id,
                    api_platform,
                    api_device_model,
                    api_os_version,
                    api_language,
                    api_request_timeout_millisecond,
                    api_db_dir,
                    api_custom_header,
                    api_keep_alive_second,
                    api_log_level,
                ))
            }
        },
    )
}
fn wire_preset_stream_impl(port_: MessagePort, instance_id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "preset_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            move |task_callback| Ok(preset_stream(api_instance_id, task_callback.stream_sink()))
        },
    )
}
fn wire_wait_stream_ready_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "wait_stream_ready",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            move |task_callback| Ok(wait_stream_ready(api_instance_id))
        },
    )
}
fn wire_set_login_info_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    token: impl Wire2Api<String> + UnwindSafe,
    user_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_login_info",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_token = token.wire2api();
            let api_user_id = user_id.wire2api();
            move |task_callback| Ok(set_login_info(api_instance_id, api_token, api_user_id))
        },
    )
}
fn wire_unset_login_info_impl(port_: MessagePort, instance_id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "unset_login_info",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            move |task_callback| Ok(unset_login_info(api_instance_id))
        },
    )
}
fn wire_user_register_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "user_register",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(user_register(api_instance_id, api_protobuf))
        },
    )
}
fn wire_user_access_token_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "user_access_token",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(user_access_token(api_instance_id, api_protobuf))
        },
    )
}
fn wire_create_robot_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "create_robot",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(create_robot(api_instance_id, api_protobuf))
        },
    )
}
fn wire_refresh_user_access_token_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "refresh_user_access_token",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(refresh_user_access_token(api_instance_id, api_protobuf))
        },
    )
}
fn wire_revoke_user_access_token_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "revoke_user_access_token",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(revoke_user_access_token(api_instance_id, api_protobuf))
        },
    )
}
fn wire_friend_apply_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "friend_apply",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(friend_apply(api_instance_id, api_protobuf))
        },
    )
}
fn wire_list_friend_apply_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "list_friend_apply",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(list_friend_apply(api_instance_id, api_protobuf))
        },
    )
}
fn wire_friend_apply_handle_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "friend_apply_handle",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(friend_apply_handle(api_instance_id, api_protobuf))
        },
    )
}
fn wire_group_create_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "group_create",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(group_create(api_instance_id, api_protobuf))
        },
    )
}
fn wire_message_send_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "message_send",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(message_send(api_instance_id, api_protobuf))
        },
    )
}
fn wire_message_batch_send_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "message_batch_send",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(message_batch_send(api_instance_id, api_protobuf))
        },
    )
}
fn wire_list_notice_impl(
    port_: MessagePort,
    instance_id: impl Wire2Api<String> + UnwindSafe,
    protobuf: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "list_notice",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_instance_id = instance_id.wire2api();
            let api_protobuf = protobuf.wire2api();
            move |task_callback| Ok(list_notice(api_instance_id, api_protobuf))
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

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<u16> for u16 {
    fn wire2api(self) -> u16 {
        self
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

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
mod web {
    use super::*;
    // Section: wire functions

    #[wasm_bindgen]
    pub fn wire_new_instance(port_: MessagePort, instance_id: String) {
        wire_new_instance_impl(port_, instance_id)
    }

    #[wasm_bindgen]
    pub fn wire_destroy_instance(port_: MessagePort, instance_id: String) {
        wire_destroy_instance_impl(port_, instance_id)
    }

    #[wasm_bindgen]
    pub fn wire_init_instance(
        port_: MessagePort,
        instance_id: String,
        host: String,
        port: u16,
        ssl: bool,
        app_id: Option<String>,
        install_id: Option<String>,
        platform: i32,
        device_model: String,
        os_version: String,
        language: i32,
        request_timeout_millisecond: i32,
        db_dir: String,
        custom_header: Option<String>,
        keep_alive_second: i32,
        log_level: i32,
    ) {
        wire_init_instance_impl(
            port_,
            instance_id,
            host,
            port,
            ssl,
            app_id,
            install_id,
            platform,
            device_model,
            os_version,
            language,
            request_timeout_millisecond,
            db_dir,
            custom_header,
            keep_alive_second,
            log_level,
        )
    }

    #[wasm_bindgen]
    pub fn wire_preset_stream(port_: MessagePort, instance_id: String) {
        wire_preset_stream_impl(port_, instance_id)
    }

    #[wasm_bindgen]
    pub fn wire_wait_stream_ready(port_: MessagePort, instance_id: String) {
        wire_wait_stream_ready_impl(port_, instance_id)
    }

    #[wasm_bindgen]
    pub fn wire_set_login_info(
        port_: MessagePort,
        instance_id: String,
        token: String,
        user_id: String,
    ) {
        wire_set_login_info_impl(port_, instance_id, token, user_id)
    }

    #[wasm_bindgen]
    pub fn wire_unset_login_info(port_: MessagePort, instance_id: String) {
        wire_unset_login_info_impl(port_, instance_id)
    }

    #[wasm_bindgen]
    pub fn wire_user_register(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_user_register_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_user_access_token(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_user_access_token_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_create_robot(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_create_robot_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_refresh_user_access_token(
        port_: MessagePort,
        instance_id: String,
        protobuf: Box<[u8]>,
    ) {
        wire_refresh_user_access_token_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_revoke_user_access_token(
        port_: MessagePort,
        instance_id: String,
        protobuf: Box<[u8]>,
    ) {
        wire_revoke_user_access_token_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_friend_apply(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_friend_apply_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_list_friend_apply(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_list_friend_apply_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_friend_apply_handle(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_friend_apply_handle_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_group_create(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_group_create_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_message_send(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_message_send_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_message_batch_send(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_message_batch_send_impl(port_, instance_id, protobuf)
    }

    #[wasm_bindgen]
    pub fn wire_list_notice(port_: MessagePort, instance_id: String, protobuf: Box<[u8]>) {
        wire_list_notice_impl(port_, instance_id, protobuf)
    }

    // Section: allocate functions

    // Section: related functions

    // Section: impl Wire2Api

    impl Wire2Api<String> for String {
        fn wire2api(self) -> String {
            self
        }
    }

    impl Wire2Api<Option<String>> for Option<String> {
        fn wire2api(self) -> Option<String> {
            self.map(Wire2Api::wire2api)
        }
    }

    impl Wire2Api<Vec<u8>> for Box<[u8]> {
        fn wire2api(self) -> Vec<u8> {
            self.into_vec()
        }
    }
    // Section: impl Wire2Api for JsValue

    impl Wire2Api<String> for JsValue {
        fn wire2api(self) -> String {
            self.as_string().expect("non-UTF-8 string, or not a string")
        }
    }
    impl Wire2Api<bool> for JsValue {
        fn wire2api(self) -> bool {
            self.is_truthy()
        }
    }
    impl Wire2Api<i32> for JsValue {
        fn wire2api(self) -> i32 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<Option<String>> for JsValue {
        fn wire2api(self) -> Option<String> {
            (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
        }
    }
    impl Wire2Api<u16> for JsValue {
        fn wire2api(self) -> u16 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<u8> for JsValue {
        fn wire2api(self) -> u8 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<Vec<u8>> for JsValue {
        fn wire2api(self) -> Vec<u8> {
            self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
        }
    }
}
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
mod io {
    use super::*;
    // Section: wire functions

    #[no_mangle]
    pub extern "C" fn wire_new_instance(port_: i64, instance_id: *mut wire_uint_8_list) {
        wire_new_instance_impl(port_, instance_id)
    }

    #[no_mangle]
    pub extern "C" fn wire_destroy_instance(port_: i64, instance_id: *mut wire_uint_8_list) {
        wire_destroy_instance_impl(port_, instance_id)
    }

    #[no_mangle]
    pub extern "C" fn wire_init_instance(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        host: *mut wire_uint_8_list,
        port: u16,
        ssl: bool,
        app_id: *mut wire_uint_8_list,
        install_id: *mut wire_uint_8_list,
        platform: i32,
        device_model: *mut wire_uint_8_list,
        os_version: *mut wire_uint_8_list,
        language: i32,
        request_timeout_millisecond: i32,
        db_dir: *mut wire_uint_8_list,
        custom_header: *mut wire_uint_8_list,
        keep_alive_second: i32,
        log_level: i32,
    ) {
        wire_init_instance_impl(
            port_,
            instance_id,
            host,
            port,
            ssl,
            app_id,
            install_id,
            platform,
            device_model,
            os_version,
            language,
            request_timeout_millisecond,
            db_dir,
            custom_header,
            keep_alive_second,
            log_level,
        )
    }

    #[no_mangle]
    pub extern "C" fn wire_preset_stream(port_: i64, instance_id: *mut wire_uint_8_list) {
        wire_preset_stream_impl(port_, instance_id)
    }

    #[no_mangle]
    pub extern "C" fn wire_wait_stream_ready(port_: i64, instance_id: *mut wire_uint_8_list) {
        wire_wait_stream_ready_impl(port_, instance_id)
    }

    #[no_mangle]
    pub extern "C" fn wire_set_login_info(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        token: *mut wire_uint_8_list,
        user_id: *mut wire_uint_8_list,
    ) {
        wire_set_login_info_impl(port_, instance_id, token, user_id)
    }

    #[no_mangle]
    pub extern "C" fn wire_unset_login_info(port_: i64, instance_id: *mut wire_uint_8_list) {
        wire_unset_login_info_impl(port_, instance_id)
    }

    #[no_mangle]
    pub extern "C" fn wire_user_register(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_user_register_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_user_access_token(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_user_access_token_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_create_robot(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_create_robot_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_refresh_user_access_token(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_refresh_user_access_token_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_revoke_user_access_token(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_revoke_user_access_token_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_friend_apply(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_friend_apply_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_list_friend_apply(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_list_friend_apply_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_friend_apply_handle(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_friend_apply_handle_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_group_create(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_group_create_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_message_send(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_message_send_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_message_batch_send(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_message_batch_send_impl(port_, instance_id, protobuf)
    }

    #[no_mangle]
    pub extern "C" fn wire_list_notice(
        port_: i64,
        instance_id: *mut wire_uint_8_list,
        protobuf: *mut wire_uint_8_list,
    ) {
        wire_list_notice_impl(port_, instance_id, protobuf)
    }

    // Section: allocate functions

    #[no_mangle]
    pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
        let ans = wire_uint_8_list {
            ptr: support::new_leak_vec_ptr(Default::default(), len),
            len,
        };
        support::new_leak_box_ptr(ans)
    }

    // Section: related functions

    // Section: impl Wire2Api

    impl Wire2Api<String> for *mut wire_uint_8_list {
        fn wire2api(self) -> String {
            let vec: Vec<u8> = self.wire2api();
            String::from_utf8_lossy(&vec).into_owned()
        }
    }

    impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
        fn wire2api(self) -> Vec<u8> {
            unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }
        }
    }
    // Section: wire structs

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_uint_8_list {
        ptr: *mut u8,
        len: i32,
    }

    // Section: impl NewWithNullPtr

    pub trait NewWithNullPtr {
        fn new_with_null_ptr() -> Self;
    }

    impl<T> NewWithNullPtr for *mut T {
        fn new_with_null_ptr() -> Self {
            std::ptr::null_mut()
        }
    }

    // Section: sync execution mode utility

    #[no_mangle]
    pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
        unsafe {
            let _ = support::box_from_leak_ptr(ptr);
        };
    }
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;

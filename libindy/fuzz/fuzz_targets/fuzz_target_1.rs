#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate indy;
extern crate serde_json;
extern crate libc;

use indy::api::{ErrorCode, CommandHandle};
use indy::api::wallet::{indy_create_wallet, indy_generate_wallet_key};

use std::ffi::CString;

use serde_json::json;
use libc::c_char;

fn fuzz_indy_create_wallet(data: &[u8]) {
    let command_handle: CommandHandle = 1;
    const WALLET_CONFIG: &str = r#"{"id":"indy_create_wallet_works_for_empty_type"}"#;
    let config: CString = CString::new(WALLET_CONFIG).unwrap();
    let credentials: CString = CString::new(data).unwrap();
    extern fn indy_create_wallet_cb(_command_handle_: CommandHandle, _err: ErrorCode) {}
    let _error_code = indy_create_wallet(command_handle,
                                         config.as_ptr(),
                                         credentials.as_ptr(),
                                         Some(indy_create_wallet_cb));
}

fn fuzz_indy_create_wallet_json(data: &[u8]) {
    let command_handle: CommandHandle = 1;
    const WALLET_CONFIG: &str = r#"{"id":"indy_create_wallet_works_for_empty_type"}"#;
    let config: CString = CString::new(WALLET_CONFIG).unwrap();
    extern fn indy_create_wallet_cb(_command_handle_: CommandHandle, _err: ErrorCode) {}

    let key : String = String::from_utf8_lossy(data).to_string();
    let credentials = json!({"key": key, "key_derivation_method": "RAW"}).to_string();
    let credentials: CString = CString::new(credentials).unwrap();
    let _error_code = indy_create_wallet(command_handle,
                                         config.as_ptr(),
                                         credentials.as_ptr(),
                                         Some(indy_create_wallet_cb));
}

fn fuzz_indy_generate_wallet_key(data: &[u8]) {
    let command_handle: CommandHandle = 1;
    extern fn indy_generate_wallet_key_cb(_command_handle_: CommandHandle, _err: ErrorCode, _key: *const c_char) {}
    let seed : String = String::from_utf8_lossy(data).to_string();
    let config = json!({"seed": seed}).to_string();
    let config: CString = CString::new(config).unwrap();
    let _error_code = indy_generate_wallet_key(command_handle,
                                               config.as_ptr(),
                                               Some(indy_generate_wallet_key_cb));
}

fuzz_target!(|data: &[u8]| {
    fuzz_indy_create_wallet(data);
    fuzz_indy_create_wallet_json(data);
    fuzz_indy_generate_wallet_key(data);
});

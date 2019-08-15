#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate indy;

use indy::api::{ErrorCode, CommandHandle};
use indy::api::wallet::indy_create_wallet;

use std::ffi::CString;
use std::os::raw::c_char;

fuzz_target!(|data: &[u8]| {
    let command_handle: CommandHandle = 1;
    const WALLET_CONFIG: &str = r#"{"id":"indy_create_wallet_works_for_empty_type"}"#;
    let config: *const c_char = CString::new(WALLET_CONFIG).unwrap().as_ptr();
    let credentials: *const c_char = CString::new(data).unwrap().as_ptr();
    extern fn cb(_command_handle_: CommandHandle, _err: ErrorCode) {}
    let _error_code = indy_create_wallet(command_handle,
                                 config,
                                 credentials,
                                 Some(cb));
});

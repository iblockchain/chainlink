#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;

#[no_mangle]
pub extern "C" fn sgx_http_get(url: *const u8) -> sgx_status_t {
    println!("Performing HTTP GET from within enclave with {:?}", url);
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn sgx_http_post(url: *const u8, _body: *const u8) -> sgx_status_t {
    println!("Performing HTTP POST from within enclave with {:?}", url);
    sgx_status_t::SGX_SUCCESS
}

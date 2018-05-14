#![feature(libc)]
extern crate libc;
extern crate errno;
extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use errno::{Errno, set_errno};

pub mod http;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

#[no_mangle]
pub extern "C" fn init_enclave() {
    if perform_enclave_init().is_err() {
        // Go uses the C _errno variable to get errors from C
        set_errno(Errno(1));
    }
}

fn perform_enclave_init() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t{
        secs_attr: sgx_attributes_t{
            flags: 0,
            xfrm: 0,
        },
        misc_select: 0,
    };
    let enclave = try!(SgxEnclave::create(ENCLAVE_FILE,
                                          debug,
                                          &mut launch_token,
                                          &mut launch_token_updated,
                                          &mut misc_attr));
    Ok(enclave)
}

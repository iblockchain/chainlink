#![feature(libc)]
extern crate libc;

extern crate errno;
extern crate sgx_types;
extern crate sgx_urts;

#[macro_use]
extern crate lazy_static;

use sgx_types::*;
use sgx_urts::SgxEnclave;
use errno::{Errno, set_errno};

pub mod http;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

lazy_static! {
    pub static ref ENCLAVE: SgxEnclave = {
        perform_enclave_init().unwrap_or_else(|err| {
            panic!("Failed to initialize the enclave: {}", err);
        })
    };
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
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

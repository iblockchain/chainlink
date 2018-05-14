use sgx_types::*;

use ENCLAVE;

extern {
    fn sgx_http_get(eid: sgx_enclave_id_t,
                    retval: *mut sgx_status_t,
                    url: *const u8) -> sgx_status_t;
    fn sgx_http_post(eid: sgx_enclave_id_t,
                     retval: *mut sgx_status_t,
                     url: *const u8,
                     body: *const u8) -> sgx_status_t;
}

#[no_mangle]
pub extern "C" fn http_get(url: *const u8) -> sgx_status_t {
    let mut retval = sgx_status_t::SGX_SUCCESS;
    unsafe { sgx_http_get(ENCLAVE.geteid(), &mut retval, url); }
    retval
}

#[no_mangle]
pub extern "C" fn http_post(url: *const u8, body: *const u8) -> sgx_status_t {
    let mut retval = sgx_status_t::SGX_SUCCESS;
    unsafe { sgx_http_post(ENCLAVE.geteid(), &mut retval, url, body); }
    retval
}

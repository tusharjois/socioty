use libc::{c_char, size_t};
use rand::rngs::OsRng;
use std::{boxed::Box, ffi::CStr};

use socioty::{Key, PartialEval, Tdprf};

#[no_mangle]
pub extern "C" fn tdprf_partial_eval(
    key: *const u8,
    input: *const u8,
    input_len: size_t,
) -> *mut u8 {
    let input = unsafe {
        assert!(!input.is_null());
        std::slice::from_raw_parts(input, input_len)
    };

    let key: [u8; 32] = unsafe {
        assert!(!key.is_null());
        std::slice::from_raw_parts(key, 32)
            .to_vec()
            .try_into()
            .unwrap()
    };

    let partial = Box::new(
        Tdprf::new(Key::from_bytes(key).unwrap())
            .partial_eval(input)
            .to_bytes(),
    );
    Box::into_raw(partial) as *mut u8
}
#[no_mangle]
pub extern "C" fn tdprf_generate(
    clients: *const *const c_char,
    clients_len: size_t,
    k: u32,
    output_keys: *mut *mut u8,
) {
    let mut client_strings = Vec::with_capacity(clients_len);

    unsafe {
        assert!(!clients.is_null());
        for c in std::slice::from_raw_parts(clients, clients_len).to_vec() {
            assert!(!c.is_null());
            client_strings.push(CStr::from_ptr(c).to_str().unwrap().to_string());
        }
    };

    let (_, generated) = Tdprf::generate(&mut OsRng, &client_strings, k);

    unsafe {
        for (i, g) in generated.into_iter().enumerate() {
            let ptr = Box::into_raw(Box::new(g.to_bytes())) as *mut u8;
            *output_keys.add(i) = ptr;
        }
    }
}

#[no_mangle]
pub extern "C" fn tdprf_reconstruct(
    key: *const u8,
    input: *const u8,
    input_len: size_t,
    clients: *const *const c_char,
    clients_len: size_t,
    partials: *const *mut u8,
    partials_len: size_t,
) -> *mut u8 {
    let input = unsafe {
        assert!(!input.is_null());
        std::slice::from_raw_parts(input, input_len)
    };

    let key: [u8; 32] = unsafe {
        assert!(!key.is_null());
        std::slice::from_raw_parts(key, 32)
            .to_vec()
            .try_into()
            .unwrap()
    };

    let mut partial_outputs = Vec::with_capacity(partials_len);
    unsafe {
        assert!(!partials.is_null());
        for p in std::slice::from_raw_parts(partials, partials_len).to_vec() {
            // Need to leak because the calling C function "owns" the memory now
            // So, this function can't drop the memory
            // Instead, the calling function needs to call dprf_free()
            assert!(!p.is_null());
            partial_outputs.push(
                PartialEval::from_bytes(*Box::leak(Box::from_raw(p as *mut [u8; 32]))).unwrap(),
            )
        }
    };

    let mut client_strings = Vec::with_capacity(clients_len);
    unsafe {
        assert!(!clients.is_null());
        for c in std::slice::from_raw_parts(clients, clients_len).to_vec() {
            assert!(!c.is_null());
            client_strings.push(CStr::from_ptr(c).to_str().unwrap().to_string());
        }
    };

    let output = Box::new(
        Tdprf::new(Key::from_bytes(key).unwrap())
            .reconstruct(input, &client_strings, &partial_outputs)
            .to_bytes(),
    );

    Box::into_raw(output) as *mut u8
}

#[no_mangle]
pub extern "C" fn tdprf_free(x: *mut u8) {
    unsafe {
        if x.is_null() {
            return;
        }
        Box::from_raw(x as *mut [u8; 32])
    };
}

#[no_mangle]
pub extern "C" fn tdprf_random_key() -> *mut u8 {
    Box::into_raw(Box::new(Key::random(&mut OsRng).to_bytes())) as *mut u8
}

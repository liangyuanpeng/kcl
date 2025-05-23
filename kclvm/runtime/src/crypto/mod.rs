//! Copyright The KCL Authors. All rights reserved.

extern crate blake3;
extern crate md5;
extern crate sha1;
extern crate sha2;

use core::panic;
use std::{fs::File, io::Read};

use crate::encoding::encode_text;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

use crate::*;
use uuid::Uuid;

#[allow(non_camel_case_types)]
type kclvm_value_ref_t = ValueRef;

// md5(value: str, encoding: str = "utf-8") -> str

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_md5(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(s) = get_call_arg_str(args, kwargs, 0, Some("value")) {
        let encoding = get_call_arg_str(args, kwargs, 1, Some("encoding"));
        let bytes = encode_text(&s, encoding).unwrap();
        let hex = format!("{:x}", md5::compute(bytes));
        return ValueRef::str(hex.as_ref()).into_raw(ctx);
    }
    panic!("md5() missing 1 required positional argument: 'value'");
}

// sha1(value: str, encoding: str = "utf-8") -> str

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_sha1(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(s) = get_call_arg_str(args, kwargs, 0, Some("value")) {
        let encoding = get_call_arg_str(args, kwargs, 1, Some("encoding"));
        let bytes = encode_text(&s, encoding).unwrap();
        let hex = sha1::Sha1::from(bytes).digest().to_string();
        return ValueRef::str(hex.as_ref()).into_raw(ctx);
    }
    panic!("sha1() missing 1 required positional argument: 'value'");
}

// sha224(value: str, encoding: str = "utf-8") -> str

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_sha224(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(s) = get_call_arg_str(args, kwargs, 0, Some("value")) {
        let encoding = get_call_arg_str(args, kwargs, 1, Some("encoding"));
        let bytes = encode_text(&s, encoding).unwrap();
        let mut hasher = Sha224::new();
        hasher.update(bytes);
        let result = hasher.finalize();

        let mut hex = String::with_capacity(2 * Sha256::output_size());
        use std::fmt::Write;

        for byte in result {
            let _ = write!(&mut hex, "{byte:02x}");
        }

        return ValueRef::str(hex.as_ref()).into_raw(ctx);
    }
    panic!("sha224() missing 1 required positional argument: 'value'");
}

// sha256(value: str, encoding: str = "utf-8") -> str

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_sha256(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(s) = get_call_arg_str(args, kwargs, 0, Some("value")) {
        let encoding = get_call_arg_str(args, kwargs, 1, Some("encoding"));
        let bytes = encode_text(&s, encoding).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();

        let mut hex = String::with_capacity(2 * Sha256::output_size());
        use std::fmt::Write;

        for byte in result {
            let _ = write!(&mut hex, "{byte:02x}");
        }

        return ValueRef::str(hex.as_ref()).into_raw(ctx);
    }
    panic!("sha256() missing 1 required positional argument: 'value'");
}

// sha384(value: str, encoding: str = "utf-8") -> str

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_sha384(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(s) = get_call_arg_str(args, kwargs, 0, Some("value")) {
        let encoding = get_call_arg_str(args, kwargs, 1, Some("encoding"));
        let bytes = encode_text(&s, encoding).unwrap();
        let mut hasher = Sha384::new();
        hasher.update(bytes);
        let result = hasher.finalize();

        let mut hex = String::with_capacity(2 * Sha256::output_size());
        use std::fmt::Write;

        for byte in result {
            let _ = write!(&mut hex, "{byte:02x}");
        }

        return ValueRef::str(hex.as_ref()).into_raw(ctx);
    }
    panic!("sha384() missing 1 required positional argument: 'value'");
}

// sha512(value: str, encoding: str = "utf-8") -> str

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_sha512(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(s) = get_call_arg_str(args, kwargs, 0, Some("value")) {
        let encoding = get_call_arg_str(args, kwargs, 1, Some("encoding"));
        let bytes = encode_text(&s, encoding).unwrap();
        let mut hasher = Sha512::new();
        hasher.update(bytes);
        let result = hasher.finalize();

        let mut hex = String::with_capacity(2 * Sha256::output_size());
        use std::fmt::Write;

        for byte in result {
            let _ = write!(&mut hex, "{byte:02x}");
        }

        return ValueRef::str(hex.as_ref()).into_raw(ctx);
    }
    panic!("sha512() missing 1 required positional argument: 'value'");
}

// blake3(value: str, encoding: str = "utf-8") -> str

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_blake3(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(s) = get_call_arg_str(args, kwargs, 0, Some("value")) {
        let encoding = get_call_arg_str(args, kwargs, 1, Some("encoding"));
        let bytes = encode_text(&s, encoding).unwrap();
        let hasher = blake3::hash(&bytes);

        let mut hex = String::with_capacity(2 * blake3::OUT_LEN);
        use std::fmt::Write;

        for byte in hasher.as_bytes() {
            let _ = write!(&mut hex, "{byte:02x}");
        }

        return ValueRef::str(hex.as_ref()).into_raw(ctx);
    }
    panic!("blake3() missing 1 required positional argument: 'value'");
}

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_uuid(
    ctx: *mut kclvm_context_t,
    _args: *const kclvm_value_ref_t,
    _kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let ctx = mut_ptr_as_ref(ctx);
    return ValueRef::str(Uuid::new_v4().to_string().as_ref()).into_raw(ctx);
}

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_filesha256(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(filepath) = get_call_arg_str(args, kwargs, 0, Some("filepath")) {
        // Open the file
        let mut file = File::open(&filepath)
            .unwrap_or_else(|e| panic!("failed to access file '{}': {}", filepath, e));

        // Create a SHA256 hasher instance
        let mut hasher = Sha256::new();

        // Read the file content and update the hasher
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .unwrap_or_else(|e| panic!("failed to read file '{}': {}", filepath, e));
        hasher.update(&buffer);

        // Compute the SHA256 hash
        let hash_result = hasher.finalize();

        let mut hex = String::with_capacity(2 * Sha256::output_size());
        use std::fmt::Write;

        for byte in hash_result {
            let _ = write!(&mut hex, "{byte:02x}");
        }

        return ValueRef::str(hex.as_str()).into_raw(ctx);
    }
    panic!("filesha256() missing 1 required positional argument: 'filepath'");
}

#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_filesha512(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(filepath) = get_call_arg_str(args, kwargs, 0, Some("filepath")) {
        let mut file = File::open(&filepath)
            .unwrap_or_else(|e| panic!("failed to access file '{}': {}", filepath, e));

        let mut hasher = Sha512::new();

        let mut buffer = [0; 4096];
        while let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 {
                break; // End of file
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let hash_result = hasher.finalize();

        let hex = hash_result
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();

        return ValueRef::str(&hex).into_raw(ctx);
    }
    panic!("filesha512() missing 1 required positional argument: 'filepath'");
}

// fileblake3
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_crypto_fileblake3(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(filepath) = get_call_arg_str(args, kwargs, 0, Some("filepath")) {
        let mut file = File::open(&filepath)
            .unwrap_or_else(|e| panic!("failed to access file '{}': {}", filepath, e));

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .unwrap_or_else(|e| panic!("failed to read file '{}': {}", filepath, e));

        let hasher = blake3::hash(&buffer);

        let mut hex = String::with_capacity(2 * blake3::OUT_LEN);
        use std::fmt::Write;

        for byte in hasher.as_bytes() {
            let _ = write!(&mut hex, "{byte:02x}");
        }

        return ValueRef::str(hex.as_str()).into_raw(ctx);
    }
    panic!("fileblake3() missing 1 required positional argument: 'filepath'");
}

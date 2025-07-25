/* This file was autogenerated by ctest; do not modify directly */

/// As this file is sometimes built using rustc, crate level attributes
/// are not allowed at the top-level, so we hack around this by keeping it
/// inside of a module.
mod generated_tests {
    #![allow(non_snake_case)]
    #![deny(improper_ctypes_definitions)]
    use std::ffi::CStr;
    use std::fmt::{Debug, LowerHex};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::{mem, ptr, slice};

    use super::*;

    pub static FAILED: AtomicBool = AtomicBool::new(false);
    pub static NTESTS: AtomicUsize = AtomicUsize::new(0);

    /// Check that the value returned from the Rust and C side in a certain test is equivalent.
    ///
    /// Internally it will remember which checks failed and how many tests have been run.
    fn check_same<T: PartialEq + Debug>(rust: T, c: T, attr: &str) {
        if rust != c {
            eprintln!("bad {attr}: rust: {rust:?} != c {c:?}");
            FAILED.store(true, Ordering::Relaxed);
        } else {
            NTESTS.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Check that the value returned from the Rust and C side in a certain test is equivalent.
    ///
    /// Internally it will remember which checks failed and how many tests have been run. This
    /// method is the same as `check_same` but prints errors in bytes in hex.
    fn check_same_hex<T: PartialEq + LowerHex + Debug>(rust: T, c: T, attr: &str) {
        if rust != c {
            eprintln!("bad {attr}: rust: {rust:?} ({rust:#x}) != c {c:?} ({c:#x})");
            FAILED.store(true, Ordering::Relaxed);
        } else {
            NTESTS.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Test that the value of the constant is the same in both Rust and C.
    // This performs a byte by byte comparision of the constant value.
    pub fn ctest_const_ON() {
        type T = bool;
        extern "C" {
            fn ctest_const__ON() -> *const T;
        }

        /* HACK: The slices may contian uninitialized data! We do this because
         * there isn't a good way to recursively iterate all fields. */

        let r_val: T = ON;
        let r_bytes = unsafe {
            slice::from_raw_parts(ptr::from_ref(&r_val).cast::<u8>(), size_of::<T>())
        };

        let c_bytes = unsafe {
            let c_ptr: *const T = unsafe { ctest_const__ON() };
            slice::from_raw_parts(c_ptr.cast::<u8>(), size_of::<T>())
        };

        for (i, (&b1, &b2)) in r_bytes.iter().zip(c_bytes.iter()).enumerate() {
            check_same_hex(b1, b2, &format!("ON value at byte {}", i));
        }
    }
}

use generated_tests::*;

fn main() {
    println!("RUNNING ALL TESTS");
    run_all();
    if FAILED.load(std::sync::atomic::Ordering::Relaxed) {
        panic!("some tests failed");
    } else {
        println!(
            "PASSED {} tests",
            NTESTS.load(std::sync::atomic::Ordering::Relaxed)
        );
    }
}

// Run all tests by calling the functions that define them.
fn run_all() {
    ctest_const_ON();
}

//! `extract()` = `unwrap_or_else(|| intrinsics::unreachable())`
//!
//! In a debug build: `extract()` = `unwrap_or_else(|| unreachable!())`
//!
//! Use `extract()` only when you are 200% sure that an option contains a value.

#![cfg_attr(not(debug_assertions), feature(core))]
#![deny(missing_docs)]
#![deny(warnings)]

#[cfg(not(debug_assertions))]
use std::intrinsics;

/// Extension trait for `Option` that adds the `extract()` method
pub trait Extract {
    /// The type of what's contained in the `Some` variant
    type Output;

    /// Extracts the value contained in the `Some` variant
    unsafe fn extract(self) -> Self::Output;
}

impl<T> Extract for Option<T> {
    type Output = T;

    unsafe fn extract(self) -> T {
        match self {
            #[cfg(debug_assertions)]
            None => unreachable!(),
            #[cfg(not(debug_assertions))]
            None => intrinsics::unreachable(),
            Some(x) => x,
        }
    }
}

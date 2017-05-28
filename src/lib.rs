#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

// Currently implements is_enum!(). TODO: Further explore Rust to generalize this macro to truly implement is_type!()
macro_rules! is_type {
    ($value:expr, $type_name:pat) => { if let $type_name = $value { true } else { false } };
}

#[cfg(test)]
mod unit_tests;

#![feature(c_variadic, extern_types, linkage, raw_ref_op)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

extern crate libc;

pub mod mpack_common;
pub mod mpack_platform;
pub mod mpack_reader;
pub mod mpack_writer;
pub mod mpack_expect;
pub mod mpack_node;

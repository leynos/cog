#![cfg_attr(feature = "nightly", feature(proc_macro))]

#[macro_use] extern crate diesel;
#[cfg(feature = "nightly")]
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate rustc_serialize;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));


pub mod schema;
pub mod models;


pub mod event;
pub mod service;

extern crate capnp;
pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}
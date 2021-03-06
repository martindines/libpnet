// Copyright (c) 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-test
// ignored as this error message isn't implemented yet

#![feature(custom_attribute, plugin)]
#![plugin(pnet_macros)]

extern crate pnet;

#[packet]
pub struct PacketWithPayload {
    banana: u8,
    #[length_fn = "length_fn"]
    var_length: Vec<u8>, //~ ERROR: length_fn must be of type &PacketWithPayloadHeader -> usize
    #[payload]
    payload: Vec<u8>
}

fn length_fn(_: ()) -> usize {
    unimplemented!()
}

fn main() {}

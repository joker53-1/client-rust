// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

#![allow(clippy::large_enum_variant)]

pub use protos::*;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

//! RISC-V Settings.

use crate::settings::{self, detail, Builder, Value};
use core::fmt;

// Include code generated by `cranelift-codegen/meta/src/gen_settings.rs:`. This file contains a
// public `Flags` struct with an impl for all of the settings defined in
include!(concat!(env!("OUT_DIR"), "/settings-riscv.rs"));

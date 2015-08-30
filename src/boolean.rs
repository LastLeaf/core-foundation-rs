// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Boolean type.

use base::{CFRelease, CFTypeID, TCFType};
use std::mem;

pub type Boolean = u32;

#[repr(C)]
struct __CFBoolean;

pub type CFBooleanRef = *const __CFBoolean;

/// A Boolean type.
///
/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFBoolean(CFBooleanRef);

impl Drop for CFBoolean {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFBoolean, CFBooleanRef, CFBooleanGetTypeID);

impl CFBoolean {
    pub fn true_value() -> CFBoolean {
        unsafe {
            TCFType::wrap_under_get_rule(kCFBooleanTrue)
        }
    }

    pub fn false_value() -> CFBoolean {
        unsafe {
            TCFType::wrap_under_get_rule(kCFBooleanFalse)
        }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    static kCFBooleanTrue: CFBooleanRef;
    static kCFBooleanFalse: CFBooleanRef;

    fn CFBooleanGetTypeID() -> CFTypeID;
}


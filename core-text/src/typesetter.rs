// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::attributed_string::CFAttributedStringRef;
use core_foundation::base::{CFTypeID, TCFType};
use core_foundation::mach_port::CFIndex;
use std::os::raw::c_void;

#[repr(C)]
pub struct __CTTypesetter(c_void);

pub type CTTypesetterRef = *const __CTTypesetter;

declare_TCFType! {
    CTTypesetter, CTTypesetterRef
}
impl_TCFType!(CTTypesetter, CTTypesetterRef, CTTypesetterGetTypeID);
impl_CFTypeDescription!(CTTypesetter);

impl CTTypesetter {
    pub fn new_with_attributed_string(string: CFAttributedStringRef) -> Self {
        unsafe {
            let ptr = CTTypesetterCreateWithAttributedString(string);
            CTTypesetter::wrap_under_create_rule(ptr)
        }
    }

    pub fn suggest_cluster_break(&self, start_index: CFIndex, width: f64) -> CFIndex {
        unsafe {
            CTTypesetterSuggestClusterBreak(self.0, start_index, width)
        }
    }

    pub fn suggest_cluster_break_with_offset(&self, start_index: CFIndex, width: f64, offset: f64) -> CFIndex {
        unsafe {
            CTTypesetterSuggestClusterBreakWithOffset(self.0, start_index, width, offset)
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreText", kind = "framework"))]
extern "C" {
    fn CTTypesetterGetTypeID() -> CFTypeID;
    fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTTypesetterRef;
    fn CTTypesetterSuggestClusterBreak(
        framesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: f64,
    ) -> CFIndex;
    fn CTTypesetterSuggestClusterBreakWithOffset(
        framesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: f64,
        offset: f64,
    ) -> CFIndex;
}

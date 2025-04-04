// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::serialize::error::SerializeError;
use crate::str::{unicode_to_str, unicode_to_str_via_ffi};

use serde::ser::{Serialize, Serializer};

#[repr(transparent)]
pub struct StrSerializer {
    ptr: *mut pyo3_ffi::PyObject,
}

impl StrSerializer {
    pub fn new(ptr: *mut pyo3_ffi::PyObject) -> Self {
        StrSerializer { ptr: ptr }
    }
}

impl Serialize for StrSerializer {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let uni = {
            let tmp = unicode_to_str(self.ptr);
            if unlikely!(tmp.is_none()) {
                err!(SerializeError::InvalidStr)
            }
            tmp.unwrap()
        };
        serializer.serialize_str(uni)
    }
}

#[repr(transparent)]
pub struct StrSubclassSerializer {
    ptr: *mut pyo3_ffi::PyObject,
}

impl StrSubclassSerializer {
    pub fn new(ptr: *mut pyo3_ffi::PyObject) -> Self {
        StrSubclassSerializer { ptr: ptr }
    }
}

impl Serialize for StrSubclassSerializer {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let uni = unicode_to_str_via_ffi(self.ptr);
        if unlikely!(uni.is_none()) {
            err!(SerializeError::InvalidStr)
        }
        serializer.serialize_str(uni.unwrap())
    }
}

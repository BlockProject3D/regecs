// Copyright (c) 2024, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::ffi::{CStr, CString, OsStr, OsString};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use crate::reflection::Identifier;

#[derive(Copy, Clone)]
pub struct Property {
    pub name: &'static str,
    pub identifier: Identifier,
    pub ty: &'static str
}

pub trait Type {
    const NAME: &'static str;
    const SUPER_NAME: Option<&'static str>;
    type DerefTarget: ?Sized;
}

impl<T: Type> Type for Vec<T> {
    const NAME: &'static str = T::NAME;
    const SUPER_NAME: Option<&'static str> = Some("Vec");
    type DerefTarget = <Vec<T> as Deref>::Target;
}

impl<T: Type> Type for Box<T> {
    const NAME: &'static str = T::NAME;
    const SUPER_NAME: Option<&'static str> = Some("Box");
    type DerefTarget = <Box<T> as Deref>::Target;
}

impl<T: Type> Type for Rc<T> {
    const NAME: &'static str = T::NAME;
    const SUPER_NAME: Option<&'static str> = Some("Rc");
    type DerefTarget = <Rc<T> as Deref>::Target;
}

impl<T: Type> Type for Arc<T> {
    const NAME: &'static str = T::NAME;
    const SUPER_NAME: Option<&'static str> = Some("Vec");
    type DerefTarget = <Arc<T> as Deref>::Target;
}

macro_rules! impl_type {
    (
        $($type: ty: $deref: ty => $name: literal),*
    ) => {
        $(
            impl Type for $type {
                const NAME: &'static str = $name;
                const SUPER_NAME: Option<&'static str> = None;
                type DerefTarget = $deref;
            }
        )*
    };
}

impl_type!(
    i8: i8 => "i8",
    i16: i16 => "i16",
    i32: i32 => "i32",
    i64: i64 => "i64",
    u8: u8 => "u8",
    u16: u16 => "u16",
    u32: u32 => "u32",
    u64: u64 => "u64",
    f32: f32 => "f32",
    f64: f64 => "f64",
    bool: bool => "bool",
    String: str => "String",
    PathBuf: Path => "Path",
    OsString: OsStr => "OsStr",
    CString: CStr => "CStr"
);

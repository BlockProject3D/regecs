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
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

pub trait Error {
    fn undefined_property() -> Self;
}

pub trait Value: Sized {
    type ParseError: Error;
    type LoadError: Error;
}

pub trait GetProp<'a, Prop> {
    fn get_prop(prop: &'a Prop) -> Self;
}

impl<'a, T: Clone> GetProp<'a, T> for T {
    fn get_prop(prop: &'a T) -> Self {
        prop.clone()
    }
}

impl<'a, T> GetProp<'a, T> for &'a T {
    fn get_prop(prop: &'a T) -> Self {
        prop
    }
}

impl<'a, T> GetProp<'a, Vec<T>> for &'a [T] {
    fn get_prop(prop: &'a Vec<T>) -> Self {
        &*prop
    }
}

impl<'a> GetProp<'a, String> for &'a [u8] {
    fn get_prop(prop: &'a String) -> Self {
        prop.as_ref()
    }
}

impl<'a, T> GetProp<'a, Box<T>> for &'a T {
    fn get_prop(prop: &'a Box<T>) -> Self {
        &*prop
    }
}

impl<'a, T> GetProp<'a, Rc<T>> for &'a T {
    fn get_prop(prop: &'a Rc<T>) -> Self {
        &*prop
    }
}

impl<'a, T> GetProp<'a, Arc<T>> for &'a T {
    fn get_prop(prop: &'a Arc<T>) -> Self {
        &*prop
    }
}

macro_rules! get_prop {
    ($($ptype: ty => $pborrowed: ty),*) => {
        $(
            impl<'a> GetProp<'a, $ptype> for &'a $pborrowed {
                fn get_prop(prop: &'a $ptype) -> &'a $pborrowed {
                    &*prop
                }
            }
        )*
    };
}

get_prop! {
    String => str,
    OsString => OsStr,
    CString => CStr,
    PathBuf => Path
}

pub trait ValueParser<T>: Value {
    fn parse(self) -> Result<T, Self::ParseError>;
    fn load<'a>(self, value: impl GetProp<'a, T>) -> Result<Self, Self::LoadError>;
}

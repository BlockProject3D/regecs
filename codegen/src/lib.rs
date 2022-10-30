// Copyright (c) 2022, BlockProject 3D
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

extern crate core;

mod clear;
mod dispatch;
mod fields_enum;
mod r#impl;
mod new_impl;
mod object_impl;

use crate::new_impl::NewImpl;
use crate::object_impl::ObjectImpl;
use crate::r#impl::Impl;
use clear::ClearImpl;
use proc_macro::{self, TokenStream};
use quote::ToTokens;
use syn::{parse_macro_input, Attribute, DeriveInput, Type};

fn get_context(attrs: impl Iterator<Item = Attribute>) -> Type {
    attrs
        .filter_map(|v| {
            if v.path.clone().into_token_stream().to_string() == "context" {
                Some(v.parse_args::<Type>().expect("failed to parse context"))
            } else {
                None
            }
        })
        .last()
        .expect("missing context")
}

#[proc_macro_derive(Object, attributes(context))]
pub fn object(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);
    let context = get_context(attrs.into_iter());
    ObjectImpl::parse_data((context, ident), data)
        .into_token_stream()
        .into()
}

#[proc_macro_derive(New, attributes(context))]
pub fn new(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        data,
        vis,
        ..
    } = parse_macro_input!(input);
    let context = get_context(attrs.into_iter());
    NewImpl::parse_data((context, ident, vis), data)
        .into_token_stream()
        .into()
}

#[proc_macro_derive(Clear, attributes(no_clear))]
pub fn clear(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    ClearImpl::parse_data(ident, data)
        .into_token_stream()
        .into()
}

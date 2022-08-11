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

use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{Field, Type, Variant, Visibility};
use crate::dispatch::DispatchParser;
use crate::Impl;

mod build_arguments;
mod inner;
mod trait_body;

pub struct NewImpl {
    name: Ident,
    inner: inner::Inner,
    is_enum: bool,
    vis: Visibility,
    parser: DispatchParser
}

impl Impl for NewImpl {
    type Params = (Type, Ident, Visibility);

    fn new((context, name, vis): Self::Params) -> Self {
        Self {
            inner: inner::Inner { context },
            name,
            vis,
            is_enum: false,
            parser: DispatchParser::new()
        }
    }

    fn parse_variant(&mut self, v: Variant) {
        self.is_enum = true;
        self.parser.parse_variant(self.name.clone(), v);
    }

    fn parse_field(&mut self, f: Field) {
        self.parser.parse_field(f);
    }

    fn into_token_stream(self) -> TokenStream {
        let vis = self.vis.clone();
        let ctx = self.inner.context.clone();
        let name = self.name.clone();
        let args = self.build_arguments();
        let (type_arg, body) = match args.args {
            trait_body::Arguments::None => (quote! { () }, None),
            trait_body::Arguments::Enum { name, code } => (name.into_token_stream(), Some(quote! { #vis #code })),
            trait_body::Arguments::Inline(v) => (v, None),
            trait_body::Arguments::Struct { name, code } => (name.into_token_stream(), Some(quote! { #vis #code })),
        };
        let new_body = args.new_body;
        let will_update_body = args.will_update_body;
        let mut tokens = TokenStream::new();
        if let Some(body) = body {
            tokens.append_all(body);
        }
        tokens.append_all(quote! {
            impl regecs::object::New<#ctx> for #name {
                type Arguments = #type_arg;

                fn new(ctx: &mut #ctx, state: &<#ctx as regecs::system::Context>::AppState, this: regecs::object::ObjectRef, args: Self::Arguments) -> #name {
                    #new_body
                }

                fn will_update(args: &Self::Arguments) -> bool {
                    #will_update_body
                }
            }
        });
        tokens
    }
}

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

use quote::quote;
use proc_macro2::TokenStream;
use syn::{Field, Type, Ident, Variant};
use crate::dispatch::{Dispatch, DispatchParser};
use crate::r#impl::Impl;

pub struct ObjectImpl {
    context: Type,
    name: Ident,
    parser: DispatchParser,
    is_enum: bool,
    class: String
}

fn to_token_stream<F: Fn(&TokenStream) -> TokenStream>(dispatch: &Dispatch, function: F, ctx: &Type) -> TokenStream {
    match dispatch {
        Dispatch::Field(v) => {
            let ty = &v.ty;
            let tokens = function(&v.target);
            quote! { <#ty as regecs::object::Object<#ctx>>::#tokens }
        },
        Dispatch::Variant(v) => {
            let ty = &v.ty;
            let tokens = function(&v.target);
            let v1 = &v.variant;
            quote! { #v1 => <#ty as regecs::object::Object<#ctx>>::#tokens }
        },
        Dispatch::VariantMultiField(v) => {
            let v1 = &v.variant;
            let vec: Vec<TokenStream> = v.children.iter().map(|v| {
                let ty = &v.ty;
                let tokens = function(&v.target);
                quote! { <#ty as regecs::object::Object<#ctx>>::#tokens }
            }).collect();
            quote! { #v1 => { #(#vec;)* } }
        }
    }
}

impl Impl for ObjectImpl {
    type Params = (Type, Ident);

    fn new((context, name): Self::Params) -> Self {
        ObjectImpl {
            class: name.to_string(),
            context,
            name,
            parser: DispatchParser::new(),
            is_enum: false
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
        let ctx = self.context;
        let name = self.name;
        let class = self.class;
        let dispatches = self.parser.into_inner();
        let on_event: Vec<TokenStream> = dispatches.iter()
            .map(|v| to_token_stream(v, |target| quote! { on_event(#target, ctx, state, event) }, &ctx))
            .collect();
        let on_update: Vec<TokenStream> = dispatches.iter()
            .map(|v| to_token_stream(v, |target| quote! { on_update(#target, ctx, state) }, &ctx))
            .collect();
        let on_remove: Vec<TokenStream> = dispatches.iter()
            .map(|v| to_token_stream(v, |target| quote! { on_remove(#target, ctx, state) }, &ctx))
            .collect();
        let body = match self.is_enum {
            true => quote! {
                fn on_event(&mut self, ctx: &mut #ctx, state: &<#ctx as regecs::system::Context>::AppState, event: &regecs::event::Event<<#ctx as regecs::system::Context>::Event>) {
                    match self {
                        #(#on_event,)*
                    }
                }
                fn on_remove(&mut self, ctx: &mut #ctx, state: &<#ctx as regecs::system::Context>::AppState) {
                    match self {
                        #(#on_remove,)*
                    }
                }
                fn on_update(&mut self, ctx: &mut #ctx, state: &<#ctx as regecs::system::Context>::AppState) {
                    match self {
                        #(#on_update,)*
                    }
                }
            },
            false => quote! {
                fn on_event(&mut self, ctx: &mut #ctx, state: &<#ctx as regecs::system::Context>::AppState, event: &regecs::event::Event<<#ctx as regecs::system::Context>::Event>) {
                    #(#on_event;)*
                }
                fn on_remove(&mut self, ctx: &mut #ctx, state: &<#ctx as regecs::system::Context>::AppState) {
                    #(#on_remove;)*
                }
                fn on_update(&mut self, ctx: &mut #ctx, state: &<#ctx as regecs::system::Context>::AppState) {
                    #(#on_update;)*
                }
            }
        };
        quote! {
            impl regecs::object::Object<#ctx> for #name {
                #body
                fn class(&self) -> &str {
                    #class
                }
            }
        }
    }
}

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

use std::collections::HashSet;

use proc_macro2::{TokenStream, Ident};
use quote::quote;
use syn::{Variant, Field};

use crate::{r#impl::Impl, dispatch::{DispatchParser, Dispatch}};

fn to_token_stream(dispatch: &Dispatch) -> TokenStream {
    match dispatch {
        Dispatch::Field(v) => {
            let ty = &v.ty;
            let target = &v.target;
            quote! { <#ty as regecs::component::Clear>::clear(#target, entity) }
        },
        Dispatch::Variant(v) => {
            let ty = &v.ty;
            let target = &v.target;
            let v1 = &v.variant;
            quote! { #v1 => <#ty as regecs::component::Clear>::clear(#target, entity) }
        },
        Dispatch::VariantMultiField(v) => {
            let v1 = &v.variant;
            let vec: Vec<TokenStream> = v.children.iter().map(|v| {
                let ty = &v.ty;
                let target = &v.target;
                quote! { <#ty as regecs::component::Clear>::clear(#target, entity) }
            }).collect();
            quote! { #v1 => { #(#vec;)* } }
        }
    }
}

struct Clear {
    name: Ident,
    parser: DispatchParser,
    no_clear: HashSet<String>,
    is_enum: bool
}

impl Impl for Clear {
    type Params = Ident;

    fn new(name: Self::Params) -> Self {
        Self {
            name,
            parser: DispatchParser::new(),
            no_clear: HashSet::new(),
            is_enum: false
        }
    }

    fn parse_variant(&mut self, v: Variant) {
        self.is_enum = true;
        if v.attrs.iter().any(|v| v.path.segments.last().map(|v| v.ident.to_string()) == Some("no_clear".into())) {
            self.no_clear.insert(v.ident.to_string());
        }
        self.parser.parse_variant(self.name.clone(), v);
    }

    fn parse_field(&mut self, f: Field) {
        if f.ident.is_some() && f.attrs.iter().any(|v| v.path.segments.last().map(|v| v.ident.to_string()) == Some("no_clear".into())) {
            self.no_clear.insert(f.ident.as_ref().unwrap().to_string());
        }
        self.parser.parse_field(f);
    }

    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let dispatches = self.parser.into_inner();
        let tokens = dispatches.iter().map(|v| to_token_stream(v));
        if self.is_enum {
            quote! {
                impl regecs::component::Clear for #name {
                    fn clear(&mut self, entity: regecs::object::ObjectRef) {
                        match self {
                            #(#tokens,)*
                        }
                    }
                }
            }    
        } else {
            quote! {
                impl regecs::component::Clear for #name {
                    fn clear(&mut self, entity: regecs::object::ObjectRef) {
                        #(#tokens;)*
                    }
                }
            }    
        }
    }
}

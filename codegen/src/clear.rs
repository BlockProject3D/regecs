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

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, Variant};

use crate::{
    dispatch::{Dispatch, DispatchParser, FieldName},
    r#impl::Impl,
};

fn to_token_stream(dispatch: &Dispatch, no_clear: &HashSet<FieldName>) -> Option<TokenStream> {
    match dispatch {
        Dispatch::Field(v) => {
            let ty = &v.ty;
            let target = &v.target;
            if no_clear.contains(&v.name) {
                None
            } else {
                Some(quote! { <#ty as regecs::component::Clear>::clear(#target, entity) })
            }
        },
        Dispatch::Variant(v) => {
            let ty = &v.ty;
            let target = &v.target;
            let v1 = &v.variant;
            if no_clear.contains(&FieldName::Ident(v.variant_name.clone())) {
                None
            } else {
                Some(quote! { #v1 => <#ty as regecs::component::Clear>::clear(#target, entity) })
            }
        },
        Dispatch::VariantMultiField(v) => {
            let v1 = &v.variant;
            let vec: Vec<TokenStream> = v
                .children
                .iter()
                .map(|v| {
                    let ty = &v.ty;
                    let target = &v.target;
                    quote! { <#ty as regecs::component::Clear>::clear(#target, entity) }
                })
                .collect();
            if no_clear.contains(&FieldName::Ident(v.variant_name.clone())) {
                None
            } else {
                Some(quote! { #v1 => { #(#vec;)* } })
            }
        },
    }
}

pub struct ClearImpl {
    name: Ident,
    parser: DispatchParser,
    no_clear: HashSet<FieldName>,
    is_enum: bool,
}

impl Impl for ClearImpl {
    type Params = Ident;

    fn new(name: Self::Params) -> Self {
        Self {
            name,
            parser: DispatchParser::new(),
            no_clear: HashSet::new(),
            is_enum: false,
        }
    }

    fn parse_variant(&mut self, v: Variant) {
        self.is_enum = true;
        let no_clear = v.attrs.iter().any(|v| {
            v.path.segments.last().map(|v| v.ident.to_string()) == Some("no_clear".into())
        });
        let v = self.parser.parse_variant(self.name.clone(), v);
        if no_clear {
            if let Some(v) = v {
                match v {
                    Dispatch::Variant(v) => {
                        self.no_clear
                            .insert(FieldName::Ident(v.variant_name.clone()));
                    },
                    Dispatch::VariantMultiField(v) => {
                        self.no_clear
                            .insert(FieldName::Ident(v.variant_name.clone()));
                    },
                    _ => std::unreachable!(),
                }
            }
        }
    }

    fn parse_field(&mut self, f: Field) {
        let no_clear = f.attrs.iter().any(|v| {
            v.path.segments.last().map(|v| v.ident.to_string()) == Some("no_clear".into())
        });
        let f = self.parser.parse_field(f);
        if no_clear {
            self.no_clear.insert(f.name.clone());
        }
    }

    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let dispatches = self.parser.into_inner();
        let tokens = dispatches
            .iter()
            .map(|v| to_token_stream(v, &self.no_clear));
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

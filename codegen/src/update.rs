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

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, Type, Variant};
use crate::dispatch::{Dispatch, DispatchParser};
use crate::r#impl::Impl;
use crate::util::{Attributes, FlagRecorder};

fn to_token_stream(dispatch: &Dispatch, context: &Type, no_update: &FlagRecorder) -> Option<TokenStream> {
    let flagged = no_update.is_flagged(dispatch);
    match dispatch {
        Dispatch::Field(v) => {
            let ty = &v.ty;
            let target = &v.target;
            if flagged {
                None
            } else {
                Some(quote! { <#ty as regecs::system::Update<#context>>::update(#target, ctx, state) })
            }
        },
        Dispatch::Variant(v) => {
            let ty = &v.ty;
            let target = &v.target;
            let v1 = &v.variant;
            if flagged {
                None
            } else {
                Some(quote! { #v1 => <#ty as regecs::system::Update<#context>>::update(#target, ctx, state) })
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
                    quote! { <#ty as regecs::system::Update<#context>>::update(#target, ctx, state) }
                })
                .collect();
            if flagged {
                None
            } else {
                Some(quote! { #v1 => { #(#vec;)* } })
            }
        },
    }
}

pub struct UpdateImpl {
    parser: DispatchParser,
    no_update: FlagRecorder,
    name: Ident,
    context: Type
}

impl Impl for UpdateImpl {
    type Params = (Ident, Type);

    fn new((name, context): Self::Params) -> Self {
        UpdateImpl {
            parser: DispatchParser::new(),
            no_update: FlagRecorder::new(),
            name,
            context
        }
    }

    fn parse_variant(&mut self, v: Variant) {
        let no_update = v.has_attribute("no_update");
        let v = self.parser.parse_variant(self.name.clone(), v);
        if no_update {
            self.no_update.flag(v);
        }
    }

    fn parse_field(&mut self, f: Field) {
        let no_update = f.has_attribute("no_update");
        let v = self.parser.parse_field(f);
        if no_update {
            self.no_update.flag(v);
        }
    }

    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let context = self.context;
        let is_enum = self.parser.is_enum();
        let dispatches = self.parser.into_inner();
        let tokens = dispatches
            .iter()
            .map(|v| to_token_stream(v, &context, &self.no_update));
        if is_enum {
            quote! {
                impl regecs::system::Update<#context> for #name {
                    fn update(&mut self, ctx: &mut regecs::scene::state::SystemState<#context>, state: &<#context as regecs::scene::Interface>::AppState) {
                        match self {
                            #(#tokens,)*
                        }
                    }
                }
            }
        } else {
            quote! {
                impl regecs::system::Update<#context> for #name {
                    fn update(&mut self, ctx: &mut regecs::scene::state::SystemState<#context>, state: &<#context as regecs::scene::Interface>::AppState) {
                        #(#tokens;)*
                    }
                }
            }
        }
    }
}

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

use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use syn::{Field, Fields, Type, Ident, Variant, Data, Index};

struct DispatchVariant {
    variant: TokenStream,
    children: Option<Vec<Dispatch>>
}

struct Dispatch {
    target: TokenStream,
    ty: Type,
    variant: Option<DispatchVariant>
}

impl Dispatch {
    pub fn to_token_stream<F: Fn(&TokenStream) -> TokenStream>(&self, function: F, ctx: &Type) -> TokenStream {
        let ty = &self.ty;
        let tokens = function(&self.target);
        match &self.variant {
            Some(v) => {
                let v1 = &v.variant;
                match &v.children {
                    None => {
                        quote! {
                            #v1 => <#ty as regecs::object::Object<#ctx>>::#tokens
                        }
                    },
                    Some(children) => {
                        let vec: Vec<TokenStream> = children.iter().map(|v| {
                            let ty = &v.ty;
                            let tokens = function(&v.target);
                            quote! { <#ty as regecs::object::Object<#ctx>>::#tokens }
                        }).collect();
                        quote! {
                            #v1 => {
                                #(#vec;)*
                            }
                        }
                    }
                }
            },
            None => quote! {
                <#ty as regecs::object::Object<#ctx>>::#tokens
            }
        }
    }
}

pub struct ObjectImpl {
    context: Type,
    name: Ident,
    dispatches: Vec<Dispatch>,
    is_enum: bool,
    class: String
}

impl ObjectImpl {
    pub fn new(context: Type, name: Ident) -> ObjectImpl {
        ObjectImpl {
            class: name.to_string(),
            context,
            name,
            dispatches: Vec::new(),
            is_enum: false
        }
    }

    pub fn parse_data(context: Type, name: Ident, data: Data) -> ObjectImpl {
        let mut obj = ObjectImpl::new(context, name);
        match data {
            Data::Enum(e) => {
                for v in e.variants {
                    obj.parse_variant(v);
                }
            },
            Data::Struct(s) => {
                for f in s.fields {
                    obj.parse_field(f);
                }
            },
            _ => panic!("unions are not supported")
        }
        obj
    }

    pub fn parse_variant(&mut self, v: Variant) {
        let name = self.name.clone();
        let variant = v.ident;
        self.is_enum = true;
        match v.fields {
            Fields::Named(_) => {
                // Initially I thought Rust did support using a single variable to access all
                // fields of a struct variant. Unfortunately that's not possible, in the mean
                // time that I find an algorithm to generate the expansion required for these
                // variants, just panic with an "unsupported" message.
                // Also "self" is not an Ident which greatly limits code re-usability.
                //TODO: Fix
                panic!("struct enum variants are not supported")
            }
            Fields::Unnamed(v) => {
                if v.unnamed.len() > 1 {
                    // Same problem as for Fields::Named.
                    //TODO: Fix
                    panic!("tuple enum variants with more than 1 item are not supported")
                }
                if v.unnamed.len() < 1 {
                    return;
                }
                let field = v.unnamed.into_iter().last().unwrap();
                self.dispatches.push(Dispatch {
                    ty: field.ty,
                    target: quote! { v },
                    variant: Some(DispatchVariant{
                        variant: quote! { #name::#variant(v) },
                        children: None
                    })
                });
            }
            _ => ()
        }
    }

    pub fn parse_field(&mut self, f: Field) {
        let index = Index::from(self.dispatches.len());
        let name = f.ident.map(|v| v.into_token_stream()).unwrap_or(quote! { #index });
        self.dispatches.push(Dispatch {
            ty: f.ty,
            target: quote! { &mut self.#name },
            variant: None
        });
    }

    pub fn into_token_stream(self) -> TokenStream {
        let ctx = self.context;
        let name = self.name;
        let class = self.class;
        let on_event: Vec<TokenStream> = self.dispatches.iter()
            .map(|v| v.to_token_stream(|target| quote! { on_event(#target, ctx, state, event) }, &ctx))
            .collect();
        let on_update: Vec<TokenStream> = self.dispatches.iter()
            .map(|v| v.to_token_stream(|target| quote! { on_update(#target, ctx, state) }, &ctx))
            .collect();
        let on_remove: Vec<TokenStream> = self.dispatches.iter()
            .map(|v| v.to_token_stream(|target| quote! { on_remove(#target, ctx, state) }, &ctx))
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

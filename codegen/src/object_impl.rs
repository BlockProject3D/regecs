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
use proc_macro2::{Span, TokenStream};
use syn::{Field, Fields, Type, Ident, Variant, Data, Index};
use crate::dispatch::{Dispatch, FieldDispatch, MultiFieldVariantDispatch, VariantDispatch};
use crate::fields_enum::{expand_named_fields, expand_unnamed_fields};

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
            Fields::Named(v) => {
                let fields = expand_named_fields(&v);
                let children: Vec<FieldDispatch> = v.named.into_iter().map(|v| {
                    FieldDispatch {
                        target: v.ident.unwrap().into_token_stream(),
                        ty: v.ty,
                    }
                }).collect();
                self.dispatches.push(Dispatch::VariantMultiField(MultiFieldVariantDispatch {
                    variant: quote! { #name::#variant #fields },
                    children
                }));
            }
            Fields::Unnamed(v) => {
                if v.unnamed.len() > 1 {
                    let fields = expand_unnamed_fields(&v);
                    let children: Vec<FieldDispatch> = v.unnamed.into_iter().enumerate().map(|(i, v)| {
                        FieldDispatch {
                            target: Ident::new(&format!("v{}", i), Span::call_site()).into_token_stream(),
                            ty: v.ty
                        }
                    }).collect();
                    self.dispatches.push(Dispatch::VariantMultiField(MultiFieldVariantDispatch {
                        variant: quote! { #name::#variant #fields },
                        children
                    }));
                    return;
                }
                if v.unnamed.len() < 1 {
                    return;
                }
                let field = v.unnamed.into_iter().last().unwrap();
                self.dispatches.push(Dispatch::Variant(VariantDispatch {
                    ty: field.ty,
                    target: quote! { v },
                    variant: quote! { #name::#variant(v) },
                }));
            }
            _ => ()
        }
    }

    pub fn parse_field(&mut self, f: Field) {
        let index = Index::from(self.dispatches.len());
        let name = f.ident.map(|v| v.into_token_stream()).unwrap_or(quote! { #index });
        self.dispatches.push(Dispatch::Field(FieldDispatch {
            ty: f.ty,
            target: quote! { &mut self.#name }
        }));
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

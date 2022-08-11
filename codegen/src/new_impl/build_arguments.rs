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

use proc_macro2::{Ident, Span};
use quote::quote;
use crate::dispatch::{Dispatch, FieldDispatch, FieldName, MultiFieldVariantDispatch, VariantDispatch};
use super::trait_body::{TraitBody, Arguments};
use super::NewImpl;

impl NewImpl {
    fn get_args_type_name(&self) -> Ident {
        let str: String = "New".into();
        let str = str + &self.name.to_string();
        //Rust type inference is really badly broken: Into is unable to see that the expected type is String so by
        // definition Into must be returning String; but no Rust type resolver is too stupid to understand that.
        // Sometimes I'm asking why is Rust devs trying to implement such badly broken traits like Into which can't
        // even reliably work half the time!!!!!!!!
        Ident::new(&str, Span::call_site())
    }

    pub fn build_arguments(self) -> TraitBody {
        let name = self.get_args_type_name();
        let dispatches = self.parser.into_inner();
        let ctx = &self.inner.context;
        if dispatches.len() == 1 {
            let dispatch = dispatches.last().unwrap();
            match dispatch {
                Dispatch::Field(v) => {
                    let ty = &v.ty;
                    match &v.name {
                        FieldName::Ident(name) => TraitBody {
                            args: Arguments::Inline(quote! { <#ty as regecs::object::New<#ctx>>::Arguments }),
                            new_body: quote! { Self { #name: <#ty as regecs::object::New<#ctx>>::new(ctx, state, this, args) } },
                            will_update_body: quote! { <#ty as regecs::object::New<#ctx>>::will_update(args) },
                        },
                        FieldName::Index(_) => TraitBody {
                            args: Arguments::Inline(quote! { <#ty as regecs::object::New<#ctx>>::Arguments }),
                            new_body: quote! { Self(<#ty as regecs::object::New<#ctx>>::new(ctx, state, this, args)) },
                            will_update_body: quote! { <#ty as regecs::object::New<#ctx>>::will_update(args) },
                        }
                    }
                },
                Dispatch::Variant(v) => {
                    let ty = &v.ty;
                    let variant_name = &v.variant_name;
                    TraitBody {
                        args: Arguments::Inline(quote! { <#ty as regecs::object::New<#ctx>>::Arguments }),
                        new_body: quote! { Self::#variant_name(<#ty as regecs::object::New<#ctx>>::new(ctx, state, this, args)) },
                        will_update_body: quote! { <#ty as regecs::object::New<#ctx>>::will_update(args) },
                    }
                },
                Dispatch::VariantMultiField(v) => {
                    if v.children.iter().any(|v| v.name.is_ident()) {
                        //All fields must be idents otherwise it's a compile error.
                        let items = v.children.iter()
                            .map(|v| self.inner.get_args(&v.ty, Some(v.name.to_ident())));
                        let new_calls = v.children.iter().map(|v| self.inner.get_new(&v.ty, &v.name));
                        let will_update_calls = v.children.iter().map(|v| self.inner.get_will_update(&v.ty, &v.name));
                        let variant_name = &v.variant_name;
                        TraitBody {
                            args: Arguments::Struct {
                                code: quote! {
                                    struct #name {
                                        #(#items,)*
                                    }
                                },
                                name
                            },
                            new_body: quote! {
                                Self::#variant_name {
                                    #(#new_calls,)*
                                }
                            },
                            will_update_body: quote! {
                                #(#will_update_calls)*
                                false
                            }
                        }
                    } else {
                        //All fields must be indices otherwise it's a compile error.
                        let items = v.children.iter().map(|v| self.inner.get_args(&v.ty, None));
                        let new_calls = v.children.iter().map(|v| self.inner.get_new(&v.ty, &v.name));
                        let will_update_calls = v.children.iter().map(|v| self.inner.get_will_update(&v.ty, &v.name));
                        let variant_name = &v.variant_name;
                        TraitBody {
                            args: Arguments::Struct {
                                code: quote! {
                                    struct #name {
                                        #(#items,)*
                                    }
                                },
                                name
                            },
                            new_body: quote! { Self::#variant_name(#(#new_calls,)*) },
                            will_update_body: quote! {
                                #(#will_update_calls)*
                                false
                            }
                        }
                    }
                }
            }
        } else if dispatches.len() > 1 {
            if self.is_enum {
                let items = dispatches.iter().map(|v| {
                    let generated_name = &name;
                    match v {
                        Dispatch::Variant(v) => {
                            let name = &v.variant_name;
                            let ty = &v.ty;
                            Dispatch::Variant(VariantDispatch {
                                ty: ty.clone(),
                                variant: quote! { #generated_name::#name(v) },
                                target: quote! { v },
                                variant_name: name.clone()
                            })
                        },
                        Dispatch::VariantMultiField(v) => {
                            let name = &v.variant_name;
                            Dispatch::VariantMultiField(MultiFieldVariantDispatch {
                                variant: quote! { #generated_name::#name },
                                variant_name: name.clone(),
                                children: v.children.clone()
                            })
                        },
                        _ => std::unreachable!()
                    }
                });
                TraitBody::from_enum(items, name.clone(), &ctx)
            } else {
                let fields: Vec<&FieldDispatch> = dispatches.iter().map(|v| match v {
                    Dispatch::Field(v) => v,
                    _ => std::unreachable!()
                }).collect();
                if fields.iter().any(|v| v.name.is_ident()) {
                    //All fields must be idents otherwise it's a compile error.
                    let mut struct_fields = Vec::new();
                    let mut will_update_calls = Vec::new();
                    let mut new_calls = Vec::new();
                    for v in fields {
                        struct_fields.push(self.inner.get_args(&v.ty, Some(v.name.to_ident())));
                        new_calls.push(self.inner.get_new(&v.ty, &v.name));
                        will_update_calls.push(self.inner.get_will_update(&v.ty, &v.name));
                    }
                    TraitBody {
                        args: Arguments::Struct {
                            code: quote! {
                                struct #name {
                                    #(#struct_fields,)*
                                }
                            },
                            name
                        },
                        new_body: quote! {
                            Self {
                                #(#new_calls,)*
                            }
                        },
                        will_update_body: quote! {
                            #(#will_update_calls)*
                            false
                        }
                    }
                } else {
                    //All fields must be indices otherwise it's a compile error.
                    let mut tuple_types = Vec::new();
                    let mut will_update_calls = Vec::new();
                    let mut new_calls = Vec::new();
                    for v in fields {
                        tuple_types.push(self.inner.get_args(&v.ty, None));
                        will_update_calls.push(self.inner.get_will_update(&v.ty, &v.name));
                        new_calls.push(self.inner.get_new(&v.ty, &v.name));
                    }
                    TraitBody {
                        args: Arguments::Inline(quote! {
                            (#(#tuple_types,)*)
                        }),
                        new_body: quote! { Self(#(#new_calls,)*) },
                        will_update_body: quote! {
                            #(#will_update_calls)*
                            false
                        }
                    }
                }
            }
        } else {
            TraitBody::none()
        }
    }
}

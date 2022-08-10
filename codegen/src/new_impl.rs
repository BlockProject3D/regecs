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

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{Field, Index, Type, Variant, Visibility};
use crate::dispatch::{Dispatch, DispatchParser, FieldDispatch, FieldName, MultiFieldVariantDispatch, VariantDispatch};
use crate::r#impl::Impl;

enum Arguments {
    None,
    Enum {
        code: TokenStream,
        name: Ident
    },
    Inline(TokenStream),
    Struct {
        code: TokenStream,
        name: Ident
    }
}

struct TraitBody {
    args: Arguments,
    new_body: TokenStream,
    will_update_body: TokenStream
}

impl TraitBody {
    pub fn from_single_type(ty: &Type, ctx: &Type) -> Self {
        Self {
            args: Arguments::Inline(quote! { <#ty as regecs::object::New<#ctx>>::Arguments }),
            new_body: quote! { <#ty as regecs::object::New<#ctx>>::new(ctx, state, this, args) },
            will_update_body: quote! { <#ty as regecs::object::New<#ctx>>::will_update(args) },
        }
    }

    pub fn none() -> Self {
        Self {
            args: Arguments::None,
            new_body: TokenStream::new(),
            will_update_body: TokenStream::new()
        }
    }

    pub fn from_enum(variants: impl Iterator<Item = Dispatch>, name_generated: Ident, name_self: &Ident, ctx: &Type) -> Self {
        let mut new1 = Vec::new();
        let mut will_update1 = Vec::new();
        let mut variants1 = Vec::new();
        for variant in variants {
            let (new, will_update, enum_variant) = match variant {
                Dispatch::Variant(v) => {
                    let variant = &v.variant;
                    let variant_name = &v.variant_name;
                    let ty = &v.ty;
                    let new = quote! { #variant => #name_self::#variant_name(<#ty as regecs::object::New<#ctx>>::new(ctx, state, this, v)) };
                    let will_update = quote! { #variant => <#ty as regecs::object::New<#ctx>>::will_update(v) };
                    let enum_variant = quote! { #variant_name(<#ty as regecs::object::New<#ctx>>::Arguments) };
                    (new, will_update, enum_variant)
                },
                Dispatch::VariantMultiField(v) => {
                    let variant = &v.variant;
                    let variant_name = &v.variant_name;
                    let with_idents = v.children.iter().any(|v| v.name.is_ident());
                    let mut items_new = TokenStream::new();
                    let mut items_variant = TokenStream::new();
                    let mut items_will_update = TokenStream::new();
                    for v in v.children.iter() {
                        let ty = &v.ty;
                        let target = &v.target;
                        if with_idents {
                            items_variant.append_all(quote! { #target: <#ty as regecs::object::New<#ctx>>::Arguments, });
                            items_new.append_all(quote! { #target: <#ty as regecs::object::New<#ctx>>::new(ctx, state, this, #target), });
                        } else {
                            items_new.append_all(quote! { <#ty as regecs::object::New<#ctx>>::new(ctx, state, this, #target), });
                            items_variant.append_all(quote! { <#ty as regecs::object::New<#ctx>>::Arguments, });
                        }
                        items_will_update.append_all(quote! {
                            if <#ty as regecs::object::New<#ctx>>::will_update(#target) {
                                return true;
                            }
                        });
                    }
                    let (new, enum_variant) = if with_idents {
                        let new = quote! { #variant => #name_self::#variant_name { #items_new } };
                        let enum_variant = quote! { #variant_name { #items_variant } };
                        (new, enum_variant)
                    } else {
                        let new = quote! { #variant => #name_self::#variant_name(#items_new) };
                        let enum_variant = quote! { #variant_name(#items_variant) };
                        (new, enum_variant)
                    };
                    let will_update = quote! {
                        #variant => {
                            #items_will_update
                            false
                        }
                    };
                    (new, will_update, enum_variant)
                },
                _ => std::unreachable!()
            };
            new1.push(new);
            will_update1.push(will_update);
            variants1.push(enum_variant);
        }
        Self {
            args: Arguments::Enum {
                code: quote! {
                        enum #name_generated {
                            #(#variants1,)*
                        }
                    },
                name: name_generated
            },
            new_body: quote! {
                match args {
                    #(#new1,)*
                }
            },
            will_update_body: quote! {
                match args {
                    #(#will_update1,)*
                }
            }
        }
    }
}

struct Inner {
    context: Type,
}

impl Inner {
    pub fn get_args(&self, ty: &Type, name: Option<&Ident>) -> TokenStream {
        let ctx = &self.context;
        match name {
            None => quote! { <#ty as regecs::object::New<#ctx>>::Arguments },
            Some(name) => quote! { #name: <#ty as regecs::object::New<#ctx>>::Arguments }
        }
    }

    pub fn get_new(&self, ty: &Type, name: &FieldName) -> TokenStream {
        let ctx = &self.context;
        match name {
            FieldName::Ident(name) => quote! {
                #name: <#ty as regecs::object::New<#ctx>>::new(ctx, state, this, args.#name)
            },
            FieldName::Index(i) => {
                let index = Index::from(*i);
                quote! { <#ty as regecs::object::New<#ctx>>::new(ctx, state, this, args.#index) }
            },
        }
    }

    pub fn get_will_update(&self, ty: &Type, name: &FieldName) -> TokenStream {
        let ctx = &self.context;
        match name {
            FieldName::Ident(name) => quote! {
                if <#ty as regecs::object::New<#ctx>>::will_update(&args.#name) {
                    return true;
                }
            },
            FieldName::Index(i) => {
                let index = Index::from(*i);
                quote! {
                    if <#ty as regecs::object::New<#ctx>>::will_update(&args.#index) {
                        return true;
                    }
                }
            },
        }
    }
}

pub struct NewImpl {
    name: Ident,
    inner: Inner,
    is_enum: bool,
    vis: Visibility,
    parser: DispatchParser
}

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

    fn build_arguments(self) -> TraitBody {
        let name = self.get_args_type_name();
        let dispatches = self.parser.into_inner();
        let ctx = &self.inner.context;
        let motherfuckingrust = &self.inner;
        if dispatches.len() == 1 {
            let dispatch = dispatches.last().unwrap();
            let initial_name = self.name;
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
                            .map(|v| motherfuckingrust.get_args(&v.ty, Some(v.name.to_ident())));
                        let new_calls = v.children.iter().map(|v| motherfuckingrust.get_new(&v.ty, &v.name));
                        let will_update_calls = v.children.iter().map(|v| motherfuckingrust.get_will_update(&v.ty, &v.name));
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
                                #initial_name::#variant_name {
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
                        let items = v.children.iter().map(|v| motherfuckingrust.get_args(&v.ty, None));
                        let new_calls = v.children.iter().map(|v| motherfuckingrust.get_new(&v.ty, &v.name));
                        let will_update_calls = v.children.iter().map(|v| motherfuckingrust.get_will_update(&v.ty, &v.name));
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
                            new_body: quote! { #initial_name::#variant_name(#(#new_calls,)*) },
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
                TraitBody::from_enum(items, name.clone(), &self.name, &ctx)
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

impl Impl for NewImpl {
    type Params = (Type, Ident, Visibility);

    fn new((context, name, vis): Self::Params) -> Self {
        Self {
            inner: Inner { context },
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
            Arguments::None => (quote! { () }, None),
            Arguments::Enum { name, code } => (name.into_token_stream(), Some(quote! { #vis #code })),
            Arguments::Inline(v) => (v, None),
            Arguments::Struct { name, code } => (name.into_token_stream(), Some(quote! { #vis #code })),
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

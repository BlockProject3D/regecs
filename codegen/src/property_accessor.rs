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

use crate::dispatch::{DispatchParser, FieldDispatch};
use crate::r#impl::Impl;
use crate::util::Attributes;
use proc_macro2::{Ident, TokenStream};
use quote::__private::Span;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::{ExprAssign, Field, Token, Variant};

fn get_target(map: &HashMap<String, Ident>, field: &FieldDispatch) -> (TokenStream, bool) {
    let field_name = field.name.to_string();
    map.get(&field_name)
        .map(|v| {
            (
                quote! {
                        self.#v
                },
                true,
            )
        })
        .unwrap_or_else(|| {
            let ident = Ident::new(&field_name, Span::call_site());
            (quote! { self.#ident }, false)
        })
}

pub struct PropertyAccessorImpl {
    parser: DispatchParser,
    name: Ident,
    set_map: HashMap<String, Ident>,
    get_map: HashMap<String, Ident>,
}

impl PropertyAccessorImpl {
    fn gen_set_prop(&self, (index, field): (usize, FieldDispatch)) -> TokenStream {
        let (target, is_function) = get_target(&self.set_map, &field);
        let ty = field.ty;
        let setter = match is_function {
            true => quote! {
                #target(<V as regecs::reflection::property::value::ValueParser<#ty>>::parse(value)?);
            },
            false => quote! {
                #target = <V as regecs::reflection::property::value::ValueParser<#ty>>::parse(value)?;
            },
        };
        quote! {
            #index => {
                #setter
                Ok(())
            }
        }
    }

    fn gen_get_prop(&self, (index, field): (usize, FieldDispatch)) -> TokenStream {
        let (target, is_function) = get_target(&self.get_map, &field);
        let ty = field.ty;
        let getter = match is_function {
            true => quote! {
                #target()
            },
            false => quote! {
                &#target
            },
        };
        quote! {
            #index => {
                let val = #getter;
                <V as regecs::reflection::property::value::ValueParser<#ty>>::load(value, val)
            }
        }
    }
}

impl Impl for PropertyAccessorImpl {
    type Params = Ident;

    fn new(params: Self::Params) -> Self {
        Self {
            name: params,
            parser: DispatchParser::new(),
            set_map: HashMap::new(),
            get_map: HashMap::new(),
        }
    }

    fn parse_variant(&mut self, _: Variant) {
        panic!("Enums are not supported");
    }

    fn parse_field(&mut self, f: Field) {
        let attrs = f.attributes_by_name("property").next();
        if let Some(attr) = attrs {
            let args = attr.parse_args_with(Punctuated::<ExprAssign, Token![,]>::parse_terminated);
            if let Ok(args) = args {
                for kv in args {
                    let key: Ident = syn::parse2(kv.left.into_token_stream())
                        .expect("unable to parse key name as identifier");
                    let value: Ident = syn::parse2(kv.right.into_token_stream())
                        .expect("unable to parse value as identifier");
                    match &*key.to_string() {
                        "get" => {
                            self.get_map
                                .insert(f.ident.clone().unwrap().to_string(), value);
                        },
                        "set" => {
                            self.set_map
                                .insert(f.ident.clone().unwrap().to_string(), value);
                        },
                        _ => (),
                    }
                }
            }
            self.parser.parse_field(f);
        }
    }

    fn into_token_stream(mut self) -> TokenStream {
        let dispatches = self.parser.into_inner();
        self.parser = DispatchParser::new(); //required to allow re-using self for gen_set_prop
        let type_list: Vec<TokenStream> = dispatches
            .iter()
            .filter_map(|v| v.clone().into_field())
            .map(|v| {
                let ty = v.ty;
                quote! {
                    regecs::reflection::property::value::ValueParser<#ty>
                }
            })
            .collect();
        let setter_mapping: Vec<TokenStream> = dispatches
            .iter()
            .filter_map(|v| v.clone().into_field())
            .enumerate()
            .map(|v| self.gen_set_prop(v))
            .collect();
        let getter_mapping: Vec<TokenStream> = dispatches
            .iter()
            .filter_map(|v| v.clone().into_field())
            .enumerate()
            .map(|v| self.gen_get_prop(v))
            .collect();
        let name = self.name;
        let new_name = Ident::new(
            &(String::from("PropertyAccessor") + &name.to_string()),
            name.span(),
        );
        quote! {
            pub trait #new_name: #(#type_list)+* {}
            impl<T: #(#type_list)+*> #new_name for T {}

            impl<V: regecs::reflection::property::value::Value + #new_name> regecs::reflection::component::PropertyAccessor<V> for #name {
                fn set_property(&mut self, identifier: Identifier, value: V) -> Result<(), V::ParseError> {
                    match identifier.into_raw() {
                        #(#setter_mapping),*
                        _ => Err(<<V as regecs::reflection::property::value::Value>::ParseError as regecs::reflection::property::value::Error>::undefined_property())
                    }
                }

                fn get_property(&self, identifier: Identifier, value: V) -> Result<V, V::LoadError> {
                    match identifier.into_raw() {
                        #(#getter_mapping),*
                        _ => Err(<<V as regecs::reflection::property::value::Value>::LoadError as regecs::reflection::property::value::Error>::undefined_property())
                    }
                }
            }
        }
    }
}

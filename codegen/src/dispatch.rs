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

use std::fmt::{Display, Formatter};
use crate::fields_enum::{expand_named_fields, expand_unnamed_fields};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Field, Fields, Index, Type, Variant};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum FieldName {
    Ident(Ident),
    Index(usize),
}

impl Display for FieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldName::Ident(v) => write!(f, "{}", v),
            FieldName::Index(v) => write!(f, "id{}", v)
        }
    }
}

#[derive(Clone)]
pub struct FieldDispatch {
    pub name: FieldName,
    pub target: TokenStream,
    pub ty: Type,
}

#[derive(Clone)]
pub struct VariantDispatch {
    pub target: TokenStream,
    pub ty: Type,
    pub variant: TokenStream,
    pub variant_name: Ident,
}

#[derive(Clone)]
pub struct MultiFieldVariantDispatch {
    pub variant: TokenStream,
    pub variant_name: Ident,
    pub children: Vec<FieldDispatch>,
}

#[derive(Clone)]
pub enum Dispatch {
    Field(FieldDispatch),
    Variant(VariantDispatch),
    VariantMultiField(MultiFieldVariantDispatch),
}

impl Dispatch {
    pub fn into_field(self) -> Option<FieldDispatch> {
        match self {
            Dispatch::Field(f) => Some(f),
            _ => None
        }
    }
}

pub struct DispatchParser {
    dispatches: Vec<Dispatch>,
    is_enum: bool
}

impl DispatchParser {
    pub fn new() -> DispatchParser {
        DispatchParser {
            dispatches: Vec::new(),
            is_enum: false
        }
    }

    pub fn parse_variant(&mut self, type_name: Ident, v: Variant) -> Option<&Dispatch> {
        self.is_enum = true;
        let variant = v.ident;
        let dispatch = match v.fields {
            Fields::Named(v) => {
                let fields = expand_named_fields(&v);
                let children: Vec<FieldDispatch> = v
                    .named
                    .into_iter()
                    .map(|v| FieldDispatch {
                        name: FieldName::Ident(v.ident.clone().unwrap()),
                        target: v.ident.unwrap().into_token_stream(),
                        ty: v.ty,
                    })
                    .collect();
                Some(Dispatch::VariantMultiField(MultiFieldVariantDispatch {
                    variant: quote! { #type_name::#variant #fields },
                    variant_name: variant,
                    children,
                }))
            },
            Fields::Unnamed(v) => {
                if v.unnamed.len() > 1 {
                    let fields = expand_unnamed_fields(&v);
                    let children: Vec<FieldDispatch> = v
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| FieldDispatch {
                            name: FieldName::Index(i),
                            target: Ident::new(&format!("v{}", i), Span::call_site())
                                .into_token_stream(),
                            ty: v.ty,
                        })
                        .collect();
                    Some(Dispatch::VariantMultiField(MultiFieldVariantDispatch {
                        variant: quote! { #type_name::#variant #fields },
                        variant_name: variant,
                        children,
                    }))
                } else if v.unnamed.len() < 1 {
                    None
                } else {
                    let field = v.unnamed.into_iter().last().unwrap();
                    Some(Dispatch::Variant(VariantDispatch {
                        ty: field.ty,
                        target: quote! { v },
                        variant: quote! { #type_name::#variant(v) },
                        variant_name: variant,
                    }))
                }
            },
            _ => None,
        };
        if let Some(dispatch) = dispatch {
            self.dispatches.push(dispatch);
        }
        self.dispatches.last()
    }

    pub fn parse_field(&mut self, f: Field) -> &Dispatch {
        self.is_enum = false;
        let index = Index::from(self.dispatches.len());
        let name = f
            .ident
            .clone()
            .map(|v| v.into_token_stream())
            .unwrap_or(quote! { #index });
        self.dispatches.push(Dispatch::Field(FieldDispatch {
            name: f
                .ident
                .map(FieldName::Ident)
                .unwrap_or(FieldName::Index(self.dispatches.len())),
            ty: f.ty,
            target: quote! { &mut self.#name },
        }));
        unsafe { self.dispatches.last().unwrap_unchecked() }
    }

    pub fn is_enum(&self) -> bool {
        self.is_enum
    }

    pub fn into_inner(self) -> Vec<Dispatch> {
        self.dispatches
    }
}

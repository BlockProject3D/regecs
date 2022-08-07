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

use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

pub struct FieldDispatch {
    pub target: TokenStream,
    pub ty: Type
}

pub struct VariantDispatch {
    pub target: TokenStream,
    pub ty: Type,
    pub variant: TokenStream
}

pub struct MultiFieldVariantDispatch {
    pub variant: TokenStream,
    pub children: Vec<FieldDispatch>
}

pub enum Dispatch {
    Field(FieldDispatch),
    Variant(VariantDispatch),
    VariantMultiField(MultiFieldVariantDispatch)
}

impl Dispatch {
    pub fn to_token_stream<F: Fn(&TokenStream) -> TokenStream>(&self, function: F, ctx: &Type) -> TokenStream {
        match self {
            Dispatch::Field(v) => {
                let ty = &v.ty;
                let tokens = function(&v.target);
                quote! {
                    <#ty as regecs::object::Object<#ctx>>::#tokens
                }
            },
            Dispatch::Variant(v) => {
                let ty = &v.ty;
                let tokens = function(&v.target);
                let v1 = &v.variant;
                quote! {
                    #v1 => <#ty as regecs::object::Object<#ctx>>::#tokens
                }
            },
            Dispatch::VariantMultiField(v) => {
                let v1 = &v.variant;
                let vec: Vec<TokenStream> = v.children.iter().map(|v| {
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
    }
}

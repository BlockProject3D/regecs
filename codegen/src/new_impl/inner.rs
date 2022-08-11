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

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Index, Type};
use crate::dispatch::FieldName;

pub struct Inner {
    pub context: Type,
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

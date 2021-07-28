// Copyright (c) 2021, BlockProject 3D
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

use std::vec::Vec;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, Type};

#[proc_macro_derive(ComponentManager)]
pub fn component_manager(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let mut v = Vec::new();

    match data {
        Data::Struct(s) => match s.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                for f in &named {
                    match &f.ty {
                        Type::Macro(m) => {
                            let comp_type = m.mac.tokens.to_string();
                            if let Some(useless) = &f.ident {
                                v.push((useless.clone(), m.mac.clone(), comp_type));
                            } else {
                                panic!("How is it possible that you get no identifier???!!!");
                            }
                        },
                        _ => panic!("Could not identify type of component for field {:?}", f.ident)
                    }
                }
            },
            _ => panic!("Your component list must not be empty")
        },
        _ => panic!("ComponentManager cannot be implemented on non-structs")
    };
    let mut impl_base_tokens = Vec::new();
    for (field_name, pool_type, _) in &v {
        impl_base_tokens.push(quote! {
            #field_name: <#pool_type>::new()
        });
    }
    let mut impls_tokens = Vec::new();
    for (field_name, pool_type, comp_type) in &v {
        let new_ident = syn::parse_str::<Type>(&comp_type).unwrap();
        let mgr_impl_tokens = quote! {
            impl ComponentProvider<#new_ident> for #ident
            {
                fn pool(&self) -> &#pool_type
                {
                    return &self.#field_name;
                }

                fn pool_mut(&mut self) -> &mut #pool_type
                {
                    return &mut self.#field_name;
                }
            }
        };
        impls_tokens.push(mgr_impl_tokens);
    }
    let output = quote! {
        impl #ident
        {
            pub fn new() -> #ident
            {
                return #ident
                {
                    #(#impl_base_tokens,)*
                };
            }
        }

        #(#impls_tokens)*
    };
    return output.into();
}

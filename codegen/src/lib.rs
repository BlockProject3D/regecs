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
use std::string::String;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::FieldsNamed;
use proc_macro2::Span;
use syn::Fields;
use syn::Data;
use syn::Type;
use syn::PathArguments;
use syn::GenericArgument;
use syn::Ident;
use syn::Visibility;

fn expand_type_name(t: &syn::Type) -> String
{
    let mut s = String::new();

    match t
    {
        Type::Path(p) =>
        {
            for v in &p.path.segments
            {
                s.push_str(&v.ident.to_string());
                s.push_str("::");
            }
            return String::from(&s[0..s.len() - 2]);
        }
        _ => panic!("Invalid generic type name for ComponentPool")
    }
}

#[proc_macro_derive(ComponentManager)]
pub fn component_manager(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let mut v = Vec::new();

    match data
    {
        Data::Struct(s) => match s.fields
        {
            Fields::Named(FieldsNamed { named, .. }) =>
            {
                for f in &named
                {
                    match &f.ty
                    {
                        Type::Path(p) =>
                        {
                            let last = &p.path.segments.last().unwrap();
                            if let PathArguments::AngleBracketed(b) = &last.arguments
                            {
                                if let Some(v1) = b.args.first()
                                {
                                    if let GenericArgument::Type(t) = v1
                                    {
                                        let name = expand_type_name(&t);
                                        if let Some(useless) = &f.ident
                                        {
                                            v.push((useless.clone(), name));
                                        }
                                        else
                                        {
                                            panic!("How is it possible that you get no identifier???!!!")
                                        }
                                    }
                                    else
                                    {
                                        panic!("Could not identify type of component for field {:?}", f.ident);
                                    }
                                }
                                else
                                {
                                    panic!("Could not identify type of component for field {:?}", f.ident);
                                }
                            }
                        },
                        _ => panic!("Could not identify type of component for field {:?}", f.ident),
                    }
                }
            },
            _ => panic!("Your component list must not be empty")
        },
        _ => panic!("ComponentManager cannot be implemented on non-structs")
    };
    let mut impl_base_tokens = Vec::new();
    for (field_name, _) in &v
    {
        impl_base_tokens.push(
            quote!
            {
                #field_name: ComponentPool::new()
            }
        );
    };
    let mut impls_tokens = Vec::new();
    for (field_name, component_type) in &v
    {
        let mut s = component_type.clone();
        s.push_str("Provider");
        let new_ident = syn::parse_str::<Type>(&s).unwrap();
        let new_ident1 = syn::parse_str::<Type>(&component_type).unwrap();
        let mgr_impl_tokens = quote!
        {
            impl #new_ident for #ident
            {
                fn get(&mut self, id: usize) -> &mut #new_ident1
                {
                    return self.#field_name.get(id);
                }

                fn get_pool(&mut self) -> &mut ComponentPool<#new_ident1>
                {
                    return &mut self.#field_name;
                }
            }
        };
        impls_tokens.push(mgr_impl_tokens);
    }
    let output = quote!
    {
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

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, vis, .. } = parse_macro_input!(input);

    let mut s = ident.to_string();
    s.push_str("Provider");
    let new_ident = Ident::new(&s, Span::call_site());
    let vis = match vis
    {
        Visibility::Public(_) => Some(Ident::new("pub", Span::call_site())),
        _ => None
    };
    let output = quote!
    {
        #vis type #new_ident = ComponentProvider<#ident>;
    };
    return output.into();
}

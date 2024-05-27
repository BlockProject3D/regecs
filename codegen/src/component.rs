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

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, Variant};
use crate::dispatch::{DispatchParser, FieldDispatch};
use crate::r#impl::Impl;
use crate::util::Attributes;

fn to_token_stream((index, field): (usize, FieldDispatch)) -> TokenStream {
    let name = field.name.to_string();
    let ty = field.ty;
    quote! {
        #name => regecs::reflection::property::Property {
            name: #name,
            identifier: regecs::reflection::Identifier::from_raw(#index),
            ty: <#ty as regecs::reflection::property::Type>::NAME
        }
    }
}

pub struct ComponentImpl {
    parser: DispatchParser,
    name: Ident
}

impl Impl for ComponentImpl {
    type Params = Ident;

    fn new(params: Self::Params) -> Self {
        Self {
            parser: DispatchParser::new(),
            name: params
        }
    }

    fn parse_variant(&mut self, _: Variant) {
        panic!("Enums are not supported")
    }

    fn parse_field(&mut self, f: Field) {
        if f.has_attribute("property") {
            self.parser.parse_field(f);
        }
    }

    fn into_token_stream(self) -> TokenStream {
        let name=  self.name;
        let name_string = name.to_string();
        let tokens = self.parser.into_inner().into_iter()
            .filter_map(|v| v.into_field())
            .enumerate()
            .map(to_token_stream);
        quote! {
            impl regecs::reflection::component::Component for #name {
                const NAME: &'static str = #name_string;
                const PROPERTIES: &'static regecs::reflection::property::list::PropertyList = &regecs::reflection::property::list::PropertyList::new(phf::phf_map! {
                    #(#tokens),*
                });
            }
        }
    }
}

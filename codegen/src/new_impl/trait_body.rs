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
use quote::{quote, TokenStreamExt};
use syn::Type;
use crate::dispatch::Dispatch;

pub enum Arguments {
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

pub struct TraitBody {
    pub args: Arguments,
    pub new_body: TokenStream,
    pub will_update_body: TokenStream
}

impl TraitBody {
    pub fn none() -> Self {
        Self {
            args: Arguments::None,
            new_body: TokenStream::new(),
            will_update_body: TokenStream::new()
        }
    }

    pub fn from_enum(variants: impl Iterator<Item = Dispatch>, name_generated: Ident, ctx: &Type) -> Self {
        let mut new1 = Vec::new();
        let mut will_update1 = Vec::new();
        let mut variants1 = Vec::new();
        for variant in variants {
            let (new, will_update, enum_variant) = match variant {
                Dispatch::Variant(v) => {
                    let variant = &v.variant;
                    let variant_name = &v.variant_name;
                    let ty = &v.ty;
                    let new = quote! { #variant => Self::#variant_name(<#ty as regecs::object::New<#ctx>>::new(ctx, state, this, v)) };
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
                        let new = quote! { #variant => Self::#variant_name { #items_new } };
                        let enum_variant = quote! { #variant_name { #items_variant } };
                        (new, enum_variant)
                    } else {
                        let new = quote! { #variant => Self::#variant_name(#items_new) };
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

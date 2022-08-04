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

use std::collections::HashMap;
use crate::object::{Context, Object};

pub trait NewFactory<C: Context, T: Object<C>> {
    /// A provider to provide "object safe" factory traits.
    ///
    /// Note that REGECS only provides the default [AnyFactory](crate::object::registry::AnyFactory).
    fn new_factory() -> Self;
}

pub struct ClassMap<F> {
    map: HashMap<&'static str, F>
}

impl<F> ClassMap<F> {
    pub fn new(map: HashMap<&'static str, F>) -> ClassMap<F> {
        ClassMap { map }
    }

    pub fn get(&self, class: &str) -> Option<&F> {
        self.map.get(class)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &F)> {
        self.map.iter().map(|(k, v)| (*k, v))
    }
}

pub trait Registry {
    type Factory;
    fn get_class_map() -> ClassMap<Self::Factory>;
}

/*type AnyFactoryFunc<C> = Box<dyn Fn(Box<dyn Any>) -> Option<Function<C>>>;

pub struct AnyFactory<C: Context> {
    func: AnyFactoryFunc<C>
}

impl<C: Context> AnyFactory<C> {
    pub fn create(&self, params: Box<dyn Any>) -> Option<Function<C>> {
        (self.func)(params)
    }
}

impl<C: Context, T: Object<C> + Wrap<C::Object> + Factory<Function<C>>> NewFactory<C, T> for AnyFactory<C>
    where T::Parameters: 'static {
    fn new_factory() -> Self {
        AnyFactory {
            func: Box::new(move |val| {
                let v = val.downcast().ok()?;
                Some(T::create(*v))
            })
        }
    }
}*/

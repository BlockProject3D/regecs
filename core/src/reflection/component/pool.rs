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

use crate::reflection::component::list::ComponentList;
use crate::reflection::Identifier;
use crate::reflection::property::list::PropertyList;
use crate::reflection::property::value::Value;

#[derive(Copy, Clone)]
pub struct ComponentInfo {
    pub properties: &'static PropertyList,
    pub name: &'static str,
    pub identifier: Identifier
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ComponentRef {
    ty: Identifier,
    index: usize
}

impl ComponentRef {
    pub fn from_ref<P: ComponentPool, T: crate::reflection::component::Component + crate::component::Component>(r: crate::component::ComponentRef<T>) -> Self {
        ComponentRef::from_raw(P::COMPONENTS[T::NAME].identifier, r.index)
    }

    pub fn from_raw(ty: Identifier, index: usize) -> Self {
        Self {
            ty,
            index
        }
    }

    pub fn into_raw(self) -> (Identifier, usize) {
        (self.ty, self.index)
    }

    pub fn ty(&self) -> Identifier {
        self.ty
    }

    /// Converts this (type unsafe) [ComponentRef](ComponentRef) into a type safe [ComponentRef](crate::component::ComponentRef).
    ///
    /// Safety
    ///
    /// This assumes that the target type T exactly matches the type identifier of this [ComponentRef](ComponentRef).
    pub fn unchecked_into_ref<T: crate::component::Component>(self) -> crate::component::ComponentRef<T> {
        crate::component::ComponentRef::new(self.index)
    }

    /// Converts this (type unsafe) [ComponentRef](ComponentRef) into a type safe [ComponentRef](crate::component::ComponentRef).
    ///
    /// Panic
    ///
    /// This function panics if the target type T does not exactly match the type identifier of this [ComponentRef](ComponentRef).
    pub fn into_ref<P: ComponentPool, T: crate::reflection::component::Component + crate::component::Component>(self) -> crate::component::ComponentRef<T> {
        if P::COMPONENTS[T::NAME].identifier != self.ty {
            panic!("attempt to convert component refs of unrelated type");
        }
        self.unchecked_into_ref()
    }
}

pub trait ComponentPool {
    const COMPONENTS: &'static ComponentList;
}

pub trait PropertyAccessor<V: Value> {
    fn set_property(&mut self, r: ComponentRef, identifier: Identifier, value: V) -> Result<(), V::ParseError>;
    fn get_property(&self, r: ComponentRef, identifier: Identifier, value: V) -> Result<V, V::LoadError>;
}

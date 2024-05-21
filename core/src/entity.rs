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

//! REGECS entity layer.

use crate::component::ComponentPool;
use crate::component::Component;
use crate::component::store::{Iter, IterMut};

pub type EntityIndex = u32;

pub struct ComponentType<T: Component> {
    useless: std::marker::PhantomData<T>,
}

impl<T: Component> ComponentType<T> {
    pub fn new() -> ComponentType<T> {
        return ComponentType {
            useless: std::marker::PhantomData::default(),
        };
    }
}

pub trait ComponentTypeProvider<T: Component> {
    fn class() -> ComponentType<T>;
}

impl<T: Component> ComponentTypeProvider<T> for T {
    fn class() -> ComponentType<T> {
        return ComponentType::<T>::new();
    }
}

pub struct EntityHelper<'a, CP> {
    mgr: &'a mut CP,
    entity: EntityIndex,
}

pub trait EntityPart<T: Component, CP: ComponentPool<T>> {
    fn iter(&self, _: ComponentType<T>) -> Iter<T>;
    fn iter_mut(&mut self, _: ComponentType<T>) -> IterMut<T>;
    fn get_first(&self, _: ComponentType<T>) -> Option<&T>;
    fn get_first_mut(&mut self, _: ComponentType<T>) -> Option<&mut T>;
}

impl<'a, T: Component, CP: ComponentPool<T>> EntityPart<T, CP> for EntityHelper<'a, CP> {
    fn iter(&self, _: ComponentType<T>) -> Iter<T> {
        return self.mgr.store().attachments(self.entity);
    }

    fn iter_mut(&mut self, _: ComponentType<T>) -> IterMut<T> {
        return self.mgr.store_mut().attachments_mut(self.entity);
    }

    fn get_first(&self, _: ComponentType<T>) -> Option<&T> {
        self.mgr.store().attachments(self.entity).next().map(|(_, v)| v)
    }

    fn get_first_mut(&mut self, _: ComponentType<T>) -> Option<&mut T> {
        self.mgr.store_mut().attachments_mut(self.entity).next().map(|(_, v)| v)
    }
}

impl<'a, CP> EntityHelper<'a, CP> {
    pub fn new(mgr: &'a mut CP, entity: EntityIndex) -> EntityHelper<'a, CP> {
        return EntityHelper { mgr, entity };
    }
}

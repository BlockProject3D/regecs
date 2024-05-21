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

use crate::component::attachments::AttachmentsManager;
use crate::component::list::List;
use crate::component::{Clear, Component, ComponentRef};
use crate::entity::EntityIndex;
use std::ops::{Index, IndexMut};

pub struct IterMut<'a, T: Component> {
    list: &'a mut T::List,
    attachments: super::attachments::Iter<'a, T>,
}

impl<'a, T: Component> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.attachments.next()?;
        unsafe {
            let ptr = &mut self.list[index] as *mut T;
            Some((index, &mut *ptr))
        }
    }
}

pub struct Iter<'a, T: Component> {
    list: &'a T::List,
    attachments: super::attachments::Iter<'a, T>,
}

impl<'a, T: Component> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.attachments.next()?;
        Some((index, &self.list[index]))
    }
}

pub struct ComponentStore<T: Component> {
    list: T::List,
    attachments: AttachmentsManager<T>,
}

impl<T: Component> Default for ComponentStore<T>
where
    T::List: Default,
{
    fn default() -> Self {
        Self::new(T::List::default())
    }
}

impl<T: Component> AsRef<T::List> for ComponentStore<T> {
    fn as_ref(&self) -> &T::List {
        &self.list
    }
}

impl<T: Component> ComponentStore<T> {
    pub fn new(list: T::List) -> Self {
        Self {
            list,
            attachments: AttachmentsManager::new(),
        }
    }

    /// Returns a mutable reference to the underlying component [List](List).
    ///
    /// **Note: Calling raw functions such as [remove](List::remove) may result in an inconsistent
    /// attachment state. When using the underlying container, the caller must ensure to not use
    /// functions which could leave dangling attachments.**
    pub fn unchecked_list_mut(&mut self) -> &mut T::List {
        &mut self.list
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn add(&mut self, comp: T) -> ComponentRef<T> {
        ComponentRef::new(self.list.add(comp))
    }

    pub fn add_attach(&mut self, entity: EntityIndex, comp: T) -> ComponentRef<T> {
        let r = self.list.add(comp);
        self.attachments.attach(entity, ComponentRef::new(r));
        ComponentRef::new(r)
    }

    pub fn remove(&mut self, r: ComponentRef<T>) {
        self.list.remove(r.index);
        self.attachments.remove(ComponentRef::new(r.index));
    }

    pub fn attachments(&self, entity: EntityIndex) -> Iter<T> {
        Iter {
            attachments: self.attachments.list(entity),
            list: &self.list,
        }
    }

    pub fn attachments_mut(&mut self, entity: EntityIndex) -> IterMut<T> {
        IterMut {
            attachments: self.attachments.list(entity),
            list: &mut self.list,
        }
    }
}

impl<T: Component> Index<ComponentRef<T>> for ComponentStore<T> {
    type Output = T;

    fn index(&self, index: ComponentRef<T>) -> &Self::Output {
        self.list.index(index.index)
    }
}

impl<T: Component> IndexMut<ComponentRef<T>> for ComponentStore<T> {
    fn index_mut(&mut self, index: ComponentRef<T>) -> &mut Self::Output {
        self.list.index_mut(index.index)
    }
}

impl<'a, T: 'a + Component> super::list::Iter<'a, T> for ComponentStore<T>
where
    T::List: super::list::Iter<'a, T>,
{
    type Iter = <<T as Component>::List as super::list::Iter<'a, T>>::Iter;
    type IterMut = <<T as Component>::List as super::list::Iter<'a, T>>::IterMut;

    fn iter(&'a self) -> Self::Iter {
        self.list.iter()
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.list.iter_mut()
    }
}

impl<T: Component> Clear for ComponentStore<T> {
    fn clear(&mut self, entity: EntityIndex) {
        for index in self.attachments.list(entity) {
            self.list.remove(index);
        }
        self.attachments.clear(entity);
    }
}

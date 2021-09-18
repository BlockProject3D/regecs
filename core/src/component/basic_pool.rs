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

use std::{
    ops::{Index, IndexMut},
    vec::Vec
};

use crate::{
    component::{
        attachments::AttachmentsManager,
        AttachmentProvider, Component, ComponentPool, IterableComponentPool
    },
    object::ObjectRef
};

macro_rules! bcp_iterator {
    ($name: ident $(, $su: ident)?) => {
        pub struct $name<'a, TComponent: Component>
        {
            comps: &'a $($su)? Vec<Option<TComponent>>,
            pos: usize,
        }

        impl<'a, TComponent: Component> $name<'a, TComponent>
        {
            fn new(comps: &'a $($su)? Vec<Option<TComponent>>) -> $name<'a, TComponent>
            {
                return $name
                {
                    comps,
                    pos: 0,
                };
            }
        }

        impl<'a, TComponent: Component> Iterator for $name<'a, TComponent>
        {
            type Item = (usize, &'a $($su)? TComponent);

            fn next(&mut self) -> Option<Self::Item>
            {
                while self.pos < self.comps.len() && self.comps[self.pos].is_none() {
                    self.pos += 1;
                }
                macro_rules! bcp_iter_internal {
                    () => {
                        if let Some(v) = &self.comps[self.pos] {
                            return Some((self.pos, v));
                        } else {
                            return None;
                        }
                    };
                    (mut) => {
                        if let Some(v) = &mut self.comps[self.pos] {
                            unsafe
                            {
                                let ptr = v as *mut TComponent;
                                return Some((self.pos, &mut *ptr));
                            }
                        } else {
                            return None;
                        }
                    }
                }
                bcp_iter_internal!($($su)?);
            }
        }
    };
}

bcp_iterator!(BcpIterator);
bcp_iterator!(BcpIteratorMut, mut);

/// Basic component pool (stores components in a single simple array list)
///
/// *May not be optimized for rendering 3D model components*
pub struct BasicComponentPool<TComponent: Component>
{
    comps: Vec<Option<TComponent>>,
    size: usize,
    attachments: AttachmentsManager
}

impl<TComponent: Component> ComponentPool<TComponent> for BasicComponentPool<TComponent>
{
    fn new() -> Self
    {
        return BasicComponentPool {
            comps: Vec::new(),
            size: 0,
            attachments: AttachmentsManager::new()
        };
    }

    fn add(&mut self, comp: TComponent) -> usize
    {
        let mut i = 0;
        while i < self.comps.len() && self.comps[i].is_some() {
            i += 1;
        }
        if i >= self.comps.len() {
            self.comps.push(Some(comp));
        } else {
            self.comps[i] = Some(comp);
        }
        self.size += 1;
        return i;
    }

    fn remove(&mut self, id: usize)
    {
        self.comps[id] = None; //Mark slot as unclaimed
        let mut i = self.comps.len() - 1; //Trim end of array
        while i > 0 && self.comps[i].is_none() {
            self.comps.remove(i);
            i -= 1;
        }
        self.size -= 1;
        self.attachments.remove(id);
    }

    fn size(&self) -> usize
    {
        return self.size;
    }
}

impl<TComponent: Component> AttachmentProvider for BasicComponentPool<TComponent>
{
    fn attach(&mut self, entity: ObjectRef, component: usize)
    {
        self.attachments.attach(entity, component);
    }

    fn list(&self, entity: ObjectRef) -> Option<Vec<usize>>
    {
        return self.attachments.list(entity);
    }

    fn clear(&mut self, entity: ObjectRef)
    {
        if let Some(set) = self.attachments.list(entity) {
            for v in set {
                self.remove(v)
            }
            self.attachments.clear(entity);
        }
    }
}

impl<'a, TComponent: 'a + Component> IterableComponentPool<'a, TComponent> for BasicComponentPool<TComponent>
{
    type Iter = BcpIterator<'a, TComponent>;
    type IterMut = BcpIteratorMut<'a, TComponent>;

    fn iter(&'a self) -> Self::Iter
    {
        return BcpIterator::new(&self.comps);
    }

    fn iter_mut(&'a mut self) -> Self::IterMut
    {
        return BcpIteratorMut::new(&mut self.comps);
    }
}

impl<TComponent: Component> Index<usize> for BasicComponentPool<TComponent>
{
    type Output = TComponent;

    fn index(&self, index: usize) -> &Self::Output
    {
        return self.comps[index].as_ref().unwrap();
    }
}

impl<TComponent: Component> IndexMut<usize> for BasicComponentPool<TComponent>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        return self.comps[index].as_mut().unwrap();
    }
}

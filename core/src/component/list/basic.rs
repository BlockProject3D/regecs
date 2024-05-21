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
use crate::component::list::{Attachments, List, Iter};
use crate::component::{Component, ComponentRef};
use std::{
    ops::{Index, IndexMut},
    vec::Vec,
};
use crate::entity::EntityIndex;

macro_rules! bcp_iterator {
    ($name: ident $(, $su: ident)?) => {
        pub struct $name<'a, T: Component>
        {
            comps: &'a $($su)? [Option<T>],
            pos: usize,
        }

        impl<'a, T: Component> $name<'a, T>
        {
            fn new(comps: &'a $($su)? [Option<T>]) -> $name<'a, T>
            {
                return $name
                {
                    comps,
                    pos: 0,
                };
            }
        }

        impl<'a, T: Component> Iterator for $name<'a, T>
        {
            type Item = (usize, &'a $($su)? T);

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
                                let ptr = v as *mut T;
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

/// Basic component list (stores components in a single simple array list)
///
/// *May not be optimized for rendering 3D model components*
pub struct BasicComponentList<T: Component> {
    comps: Vec<Option<T>>,
    size: usize,
    attachments: AttachmentsManager<T>,
}

impl<T: Component> Default for BasicComponentList<T> {
    fn default() -> Self {
        return BasicComponentList {
            comps: Vec::new(),
            size: 0,
            attachments: AttachmentsManager::new(),
        };
    }
}

impl<T: Component> List<T> for BasicComponentList<T> {
    fn add(&mut self, comp: T) -> usize {
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
        i
    }

    fn remove(&mut self, r: usize) {
        self.comps[r] = None; //Mark slot as unclaimed
        let mut i = self.comps.len() - 1; //Trim end of array
        while i > 0 && self.comps[i].is_none() {
            self.comps.remove(i);
            i -= 1;
        }
        self.size -= 1;
        self.attachments.remove(ComponentRef::new(r));
    }

    fn len(&self) -> usize {
        self.size
    }
}

impl<T: Component> Attachments<T> for BasicComponentList<T> {
    fn attach(&mut self, entity: EntityIndex, r: ComponentRef<T>) {
        self.attachments.attach(entity, r);
    }

    fn list(&self, entity: EntityIndex) -> Option<Vec<ComponentRef<T>>> {
        return self.attachments.list(entity);
    }

    fn clear(&mut self, entity: EntityIndex) {
        if let Some(set) = self.attachments.list(entity) {
            for v in set {
                self.remove(v.index)
            }
            self.attachments.clear(entity);
        }
    }

    fn get_first_mut(&mut self, entity: EntityIndex) -> Option<&mut T> {
        if let Some(r) = self.attachments.get_first(entity) {
            Some(&mut self[r.index])
        } else {
            None
        }
    }

    fn get_first(&self, entity: EntityIndex) -> Option<&T> {
        if let Some(r) = self.attachments.get_first(entity) {
            Some(&self[r.index])
        } else {
            None
        }
    }
}

impl<'a, T: 'a + Component> Iter<'a, T> for BasicComponentList<T> {
    type Iter = BcpIterator<'a, T>;
    type IterMut = BcpIteratorMut<'a, T>;

    fn iter(&'a self) -> Self::Iter {
        return BcpIterator::new(&self.comps);
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        return BcpIteratorMut::new(&mut self.comps);
    }
}

impl<T: Component> Index<usize> for BasicComponentList<T> {
    type Output = T;

    fn index(&self, r: usize) -> &Self::Output {
        return self.comps[r].as_ref().unwrap();
    }
}

impl<T: Component> IndexMut<usize> for BasicComponentList<T> {
    fn index_mut(&mut self, r: usize) -> &mut Self::Output {
        return self.comps[r].as_mut().unwrap();
    }
}

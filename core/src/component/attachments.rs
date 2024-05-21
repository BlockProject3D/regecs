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

use std::collections::{HashMap, HashSet};

use crate::component::{Component, ComponentRef};
use crate::entity::EntityIndex;

pub struct Iter<'a, T: Component>(Option<std::collections::hash_set::Iter<'a, ComponentRef<T>>>);

impl<'a, T: Component> Iterator for Iter<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = &mut self.0 {
            v.next().map(|v| v.index)
        } else {
            None
        }
    }
}

pub struct AttachmentsManager<T: Component> {
    map: HashMap<EntityIndex, HashSet<ComponentRef<T>>>,
    inv_map: HashMap<ComponentRef<T>, EntityIndex>,
}

impl<T: Component> AttachmentsManager<T> {
    pub fn new() -> AttachmentsManager<T> {
        return AttachmentsManager {
            map: HashMap::new(),
            inv_map: HashMap::new(),
        };
    }

    pub fn remove(&mut self, r: ComponentRef<T>) {
        if let Some(entity) = self.inv_map.get(&r) {
            if let Some(set) = self.map.get_mut(entity) {
                set.remove(&r);
                self.inv_map.remove(&r);
            }
        }
    }

    pub fn attach(&mut self, entity: EntityIndex, r: ComponentRef<T>) {
        if let Some(set) = self.map.get_mut(&entity) {
            set.insert(r);
        } else {
            let mut set = HashSet::new();
            set.insert(r);
            self.map.insert(entity, set);
        }
        self.inv_map.insert(r, entity);
    }

    pub fn list(&self, entity: EntityIndex) -> Iter<T> {
        if let Some(set) = self.map.get(&entity) {
            Iter(Some(set.iter()))
        } else {
            Iter(None)
        }
    }

    pub fn clear(&mut self, entity: EntityIndex) {
        self.map.remove(&entity);
    }
}

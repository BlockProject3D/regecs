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

use std::collections::{HashSet, HashMap};
use crate::object::ObjectRef;
use crate::component::interface::AttachmentProvider;

pub struct AttachmentsManager
{
    map: HashMap<ObjectRef, HashSet<usize>>,
    inv_map: HashMap<usize, ObjectRef>
}

impl AttachmentsManager
{
    pub fn new() -> AttachmentsManager
    {
        return AttachmentsManager {
            map: HashMap::new(),
            inv_map: HashMap::new()
        }
    }

    pub fn remove(&mut self, component: usize)
    {
        if let Some(entity) = self.inv_map.get(&component) {
            if let Some(set) = self.map.get_mut(entity) {
                set.remove(&component);
                self.inv_map.remove(&component);
            }
        }
    }
}

impl AttachmentProvider for AttachmentsManager
{
    fn attach(&mut self, entity: ObjectRef, component: usize)
    {
        if let Some(set) = self.map.get_mut(&entity) {
            set.insert(component);
        } else {
            let mut set = HashSet::new();
            set.insert(component);
            self.map.insert(entity, set);
        }
        self.inv_map.insert(component, entity);
    }

    fn list(&self, entity: ObjectRef) -> Option<Vec<usize>>
    {
        if let Some(set) = self.map.get(&entity) {
            let mut vec = Vec::with_capacity(set.len());
            for v in set {
                vec.push(*v);
            }
            return Some(vec);
        }
        return None;
    }

    fn clear(&mut self, entity: ObjectRef) {
        self.map.remove(&entity);
    }
}

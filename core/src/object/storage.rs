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

use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
};

use crate::object::builder::Builder;
use crate::object::{Flags, ObjectRef};
use crate::scene::Configuration;

pub struct Tree {
    enabled: HashSet<ObjectRef>,
    by_class: HashMap<String, Vec<ObjectRef>>,
    by_id: HashMap<ObjectRef, Flags>,
    count: usize,
}

impl Tree {
    pub fn is_enabled(&self, obj: ObjectRef) -> bool {
        return self.enabled.contains(&obj);
    }

    pub fn exists(&self, obj: ObjectRef) -> bool {
        return self.by_id.contains_key(&obj);
    }

    pub fn len(&self) -> usize {
        return self.count;
    }

    pub fn enabled(&self) -> impl Iterator<Item = &ObjectRef> {
        return self.enabled.iter();
    }

    pub fn iter(&self) -> impl Iterator<Item = &ObjectRef> {
        return self.by_id.keys();
    }

    pub fn can_handle_events(&self, obj: ObjectRef) -> bool {
        self.is_enabled(obj)
            && self
                .get_flags(obj)
                .map(|v| v.is_event_aware())
                .unwrap_or(false)
    }

    pub fn by_class(&self, class: &str) -> impl Iterator<Item = &ObjectRef> {
        if let Some(v) = self.by_class.get(class) {
            v.iter()
        } else {
            [].iter()
        }
    }

    pub fn get_flags(&self, obj: ObjectRef) -> Option<&Flags> {
        self.by_id.get(&obj)
    }

    pub(crate) fn insert(&mut self, obj: ObjectRef, flags: Flags, class: &str) {
        self.by_id.insert(obj, flags);
        self.enabled.insert(obj);
        let var = self
            .by_class
            .entry(String::from(class))
            .or_insert_with(Vec::new);
        var.push(obj);
        self.count += 1;
    }

    pub(crate) fn remove(&mut self, obj: ObjectRef, class: &str) {
        self.by_id.remove(&obj);
        self.enabled.remove(&obj);
        if let Some(v) = self.by_class.get_mut(class) {
            v.retain(|s| *s != obj);
        }
        self.count -= 1;
    }

    pub(crate) fn set_enabled(&mut self, obj: ObjectRef, enabled: bool) {
        if enabled {
            self.enabled.insert(obj);
        } else {
            self.enabled.remove(&obj);
        }
    }

    pub(crate) fn new() -> Tree {
        return Tree {
            enabled: HashSet::new(),
            by_class: HashMap::new(),
            by_id: HashMap::new(),
            count: 0,
        };
    }
}

pub struct Storage<I: Configuration> {
    objects: Vec<Option<Box<<I::Builder as Builder<I>>::Object>>>,
}

impl<I: Configuration> Storage<I> {
    pub fn new() -> Storage<I> {
        Storage {
            objects: Vec::new(),
        }
    }

    pub fn insert<F: FnOnce(ObjectRef) -> Box<<I::Builder as Builder<I>>::Object>>(
        &mut self,
        func: F,
    ) -> (ObjectRef, &mut Box<<I::Builder as Builder<I>>::Object>) {
        let empty_slot = {
            let mut id = 0;
            while id < self.objects.len() && self.objects[id].is_some() {
                id += 1;
            }
            if id == self.objects.len() {
                None
            } else {
                Some(id)
            }
        };

        let obj_ref;
        if let Some(slot) = empty_slot {
            obj_ref = unsafe { ObjectRef::from_raw((slot + 1) as _) };
            self.objects[slot] = Some(func(obj_ref));
        } else {
            let id = unsafe { ObjectRef::from_raw((self.objects.len() + 1) as _) };
            obj_ref = id;
            self.objects.push(Some(func(obj_ref)));
        }
        let o = unsafe {
            self.objects[(obj_ref.into_raw() - 1) as usize]
                .as_mut()
                .unwrap_unchecked()
        };
        (obj_ref, o)
    }

    pub fn destroy(&mut self, obj: ObjectRef) {
        self.objects[(obj.into_raw() - 1) as usize] = None;
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Option<Box<<I::Builder as Builder<I>>::Object>>> {
        self.objects.iter_mut()
    }
}

impl<I: Configuration> Index<ObjectRef> for Storage<I> {
    type Output = Box<<I::Builder as Builder<I>>::Object>;

    fn index(&self, index: ObjectRef) -> &Self::Output {
        return self.objects[(index.into_raw() - 1) as usize]
            .as_ref()
            .unwrap();
    }
}

impl<I: Configuration> IndexMut<ObjectRef> for Storage<I> {
    fn index_mut(&mut self, index: ObjectRef) -> &mut Self::Output {
        return self.objects[(index.into_raw() - 1) as usize]
            .as_mut()
            .unwrap();
    }
}

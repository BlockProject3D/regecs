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
    borrow::Cow,
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut}
};

use crate::object::{Context, Object, ObjectRef};

pub struct ObjectTree
{
    enabled: HashSet<ObjectRef>,
    by_class: HashMap<String, Vec<ObjectRef>>,
    by_id: HashSet<ObjectRef>,
    count: usize
}

impl ObjectTree
{
    pub fn is_enabled(&self, obj: ObjectRef) -> bool
    {
        return self.enabled.contains(&obj);
    }

    pub fn exists(&self, obj: ObjectRef) -> bool
    {
        return self.by_id.contains(&obj);
    }

    pub fn get_count(&self) -> usize
    {
        return self.count;
    }

    pub fn get_all(&self) -> impl Iterator<Item = &ObjectRef>
    {
        return self.enabled.iter();
    }

    pub fn get_all_ignore_enable(&self) -> impl Iterator<Item = &ObjectRef>
    {
        return self.by_id.iter();
    }

    pub fn find_by_class(&self, class: &str) -> Cow<'_, [ObjectRef]>
    {
        if let Some(v) = self.by_class.get(class) {
            return Cow::from(v);
        }
        return Cow::from(Vec::new());
    }

    pub(crate) fn insert(&mut self, obj: ObjectRef, class: &str)
    {
        self.by_id.insert(obj);
        let var = self
            .by_class
            .entry(String::from(class))
            .or_insert_with(Vec::new);
        var.push(obj);
        self.count += 1;
    }

    pub(crate) fn remove(&mut self, obj: ObjectRef, class: &str)
    {
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

    fn new() -> ObjectTree
    {
        return ObjectTree {
            enabled: HashSet::new(),
            by_class: HashMap::new(),
            by_id: HashSet::new(),
            count: 0
        };
    }
}

pub struct ObjectStorage<C: Context>
{
    objects: Vec<Option<Box<dyn Object<C>>>>
}

impl<C: Context> ObjectStorage<C>
{
    pub fn new() -> (ObjectStorage<C>, ObjectTree)
    {
        return (
            ObjectStorage {
                objects: Vec::new()
            },
            ObjectTree::new()
        );
    }

    pub fn insert<F: FnOnce(ObjectRef) -> Box<dyn Object<C>>>(
        &mut self,
        func: F
    ) -> (ObjectRef, &mut Box<dyn Object<C>>)
    {
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
            obj_ref = slot as ObjectRef;
            self.objects[slot] = Some(func(obj_ref));
        } else {
            let id = self.objects.len() as ObjectRef;
            obj_ref = id;
            self.objects.push(Some(func(obj_ref)));
        }
        let o = unsafe { self.objects[obj_ref as usize].as_mut().unwrap_unchecked() };
        return (obj_ref, o);
    }

    pub fn destroy(&mut self, obj: ObjectRef) {
        self.objects[obj as usize] = None;
    }

    pub fn objects(&mut self) -> impl Iterator<Item = &mut Option<Box<dyn Object<C>>>>
    {
        return self.objects.iter_mut();
    }
}

impl<C: Context> Index<ObjectRef> for ObjectStorage<C>
{
    type Output = Box<dyn Object<C>>;

    fn index(&self, index: ObjectRef) -> &Self::Output
    {
        return self.objects[index as usize].as_ref().unwrap();
    }
}

impl<C: Context> IndexMut<ObjectRef> for ObjectStorage<C>
{
    fn index_mut(&mut self, index: ObjectRef) -> &mut Self::Output
    {
        return self.objects[index as usize].as_mut().unwrap();
    }
}

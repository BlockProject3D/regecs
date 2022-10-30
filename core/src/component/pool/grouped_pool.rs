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

use crate::component::pool::{Attachments, BasicComponentPool, ComponentPool, Iter};
use crate::component::{Component, ComponentRef};
use crate::object::ObjectRef;
use std::{
    collections::{hash_map::Values, HashMap},
    hash::Hash,
    ops::{Index, IndexMut},
};

macro_rules! gcp_iterator {
    ($name: ident $(, $su: ident)?) => {
        pub struct $name<'a, K: Sized + Eq + Hash + Copy, T: Component>
        {
            comps: &'a $($su)? BasicComponentPool<T>,
            values: Values<'a, K, Vec<ComponentRef<T>>>,
            vec: Option<&'a Vec<ComponentRef<T>>>,
            pos: usize
        }

        impl <'a, K: Sized + Eq + Hash + Copy, T: Component> $name<'a, K, T>
        {
            pub fn new(comps: &'a $($su)? BasicComponentPool<T>, values: Values<'a, K, Vec<ComponentRef<T>>>) -> $name<'a, K, T>
            {
                return $name {
                    comps,
                    values,
                    vec: None,
                    pos: 0
                }
            }
        }

        impl <'a, K: Sized + Eq + Hash + Copy, T: Component> Iterator for $name<'a, K, T>
        {
            type Item = (ComponentRef<T>, &'a $($su)? T);

            fn next(&mut self) -> Option<Self::Item>
            {
                let next_id;

                macro_rules! obtain_new {
                    () => {
                        let next = self.values.next();
                        if let Some(v) = next {
                            next_id = v[0];
                            self.pos = 1;
                            self.vec = Some(v);
                        } else {
                            return None;
                        }
                    };
                }
                if let Some(v) = self.vec {
                    if self.pos >= v.len() {
                        obtain_new!();
                    } else {
                        next_id = v[self.pos];
                        self.pos += 1;
                    }
                } else {
                    obtain_new!();
                }
                macro_rules! gcp_iter_internal {
                    () => {
                        return Some((next_id, &self.comps[next_id]));
                    };
                    (mut) => {
                        unsafe {
                            let ptr = &mut self.comps[next_id] as *mut T;
                            return Some((next_id, &mut *ptr));
                        }
                    }
                }
                gcp_iter_internal!($($su)?);
            }
        }
    };
}

gcp_iterator!(GcpIterator);
gcp_iterator!(GcpIteratorMut, mut);

/// A grouped based component pool
///
/// *The grouped component pool allows to maintain grouped components when iterating*
/// *This pool is optimized for rendering systems to reduce the number of pipeline changes*
///
/// _NOTE: The K::default() group is reserved to store components that are not yet attached to a group_
pub struct GroupComponentPool<K: Sized + Eq + Hash + Copy + Default, T: Component> {
    comps: BasicComponentPool<T>,
    map: HashMap<K, Vec<ComponentRef<T>>>,
    group_map: HashMap<ComponentRef<T>, K>,
}

impl<K: Sized + Eq + Hash + Copy + Default, T: Component> GroupComponentPool<K, T> {
    /// Update the group of a component
    ///
    /// # Arguments
    ///
    /// * `r` - the component reference
    /// * `new_group` - the new group of the component
    pub fn update_group(&mut self, r: ComponentRef<T>, new_group: K) {
        if let Some(prev) = self.group_map.get(&r) {
            if *prev == new_group {
                return;
            }
            let val = self.map.entry(*prev).or_insert_with(Vec::new);
            val.retain(|val| val != &r);
            if val.is_empty() {
                // if val.len() == 0 then remove group
                self.map.remove(prev);
            }
        }
        self.group_map.insert(r, new_group);
        let val = self.map.entry(new_group).or_insert_with(Vec::new);
        val.push(r);
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, T: Component> Default for GroupComponentPool<K, T> {
    fn default() -> Self {
        return GroupComponentPool {
            comps: BasicComponentPool::default(),
            group_map: HashMap::new(),
            map: HashMap::new(),
        };
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, T: Component> ComponentPool<T>
    for GroupComponentPool<K, T>
{
    fn add(&mut self, comp: T) -> ComponentRef<T> {
        let r = self.comps.add(comp);
        self.update_group(r, K::default());
        r
    }

    fn remove(&mut self, r: ComponentRef<T>) {
        if let Some(group) = self.group_map.get(&r) {
            let map = self.map.get_mut(group).unwrap();
            map.retain(|val| val != &r);
            if map.is_empty() {
                // if map.len() == 0 then remove group (map refers to a group of components)
                self.map.remove(group);
            }
            self.group_map.remove(&r);
        }
        self.comps.remove(r);
    }

    fn len(&self) -> usize {
        return self.comps.len();
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, T: Component> Attachments<T>
    for GroupComponentPool<K, T>
{
    fn attach(&mut self, entity: ObjectRef, r: ComponentRef<T>) {
        self.comps.attach(entity, r);
    }

    fn list(&self, entity: ObjectRef) -> Option<Vec<ComponentRef<T>>> {
        return self.comps.list(entity);
    }

    fn clear(&mut self, entity: ObjectRef) {
        if let Some(set) = self.comps.list(entity) {
            for v in set {
                self.remove(v)
            }
        }
    }

    fn get_first_mut(&mut self, entity: ObjectRef) -> Option<&mut T> {
        self.comps.get_first_mut(entity)
    }

    fn get_first(&self, entity: ObjectRef) -> Option<&T> {
        self.comps.get_first(entity)
    }
}

impl<'a, K: 'a + Sized + Eq + Hash + Copy + Default, T: 'a + Component> Iter<'a, T>
    for GroupComponentPool<K, T>
{
    type Iter = GcpIterator<'a, K, T>;
    type IterMut = GcpIteratorMut<'a, K, T>;

    fn iter(&'a self) -> Self::Iter {
        return GcpIterator::new(&self.comps, self.map.values());
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        return GcpIteratorMut::new(&mut self.comps, self.map.values());
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, T: Component> Index<ComponentRef<T>>
    for GroupComponentPool<K, T>
{
    type Output = T;

    fn index(&self, index: ComponentRef<T>) -> &Self::Output {
        return self.comps.index(index);
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, T: Component> IndexMut<ComponentRef<T>>
    for GroupComponentPool<K, T>
{
    fn index_mut(&mut self, index: ComponentRef<T>) -> &mut Self::Output {
        return self.comps.index_mut(index);
    }
}

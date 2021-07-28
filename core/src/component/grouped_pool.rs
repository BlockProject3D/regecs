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
    collections::{hash_map::Values, HashMap},
    hash::Hash,
    ops::{Index, IndexMut}
};

use crate::component::{
    interface::{Component, ComponentPool, IterableComponentPool},
    BasicComponentPool
};

macro_rules! gcp_iterator {
    ($name: ident $(, $su: ident)?) => {
        pub struct $name<'a, K: Sized + Eq + Hash + Copy, TComponent: Component>
        {
            comps: &'a $($su)? BasicComponentPool<TComponent>,
            values: Values<'a, K, Vec<usize>>,
            vec: Option<&'a Vec<usize>>,
            pos: usize
        }

        impl <'a, K: Sized + Eq + Hash + Copy, TComponent: Component> $name<'a, K, TComponent>
        {
            pub fn new(comps: &'a $($su)? BasicComponentPool<TComponent>, values: Values<'a, K, Vec<usize>>) -> $name<'a, K, TComponent>
            {
                return $name {
                    comps,
                    values,
                    vec: None,
                    pos: 0
                }
            }
        }

        impl <'a, K: Sized + Eq + Hash + Copy, TComponent: Component> Iterator for $name<'a, K, TComponent>
        {
            type Item = (usize, &'a $($su)? TComponent);

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
                            let ptr = &mut self.comps[next_id] as *mut TComponent;
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
pub struct GroupComponentPool<K: Sized + Eq + Hash + Copy + Default, TComponent: Component>
{
    comps: BasicComponentPool<TComponent>,
    map: HashMap<K, Vec<usize>>,
    group_map: HashMap<usize, K>
}

impl<K: Sized + Eq + Hash + Copy + Default, TComponent: Component> GroupComponentPool<K, TComponent>
{
    /// Update the group of a component
    ///
    /// # Arguments
    ///
    /// * `id` - the component index
    /// * `new_group` - the new group of the component
    pub fn update_group(&mut self, id: usize, new_group: K)
    {
        if let Some(prev) = self.group_map.get(&id) {
            if *prev == new_group {
                return;
            }
            let val = self.map.entry(*prev).or_insert(Vec::new());
            val.retain(|val| *val != id);
            if val.len() == 0 {
                self.map.remove(prev);
            }
        }
        self.group_map.insert(id, new_group);
        let val = self.map.entry(new_group).or_insert(Vec::new());
        val.push(id);
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, TComponent: Component> ComponentPool<TComponent>
    for GroupComponentPool<K, TComponent>
{
    fn new() -> Self
    {
        return GroupComponentPool {
            comps: BasicComponentPool::new(),
            group_map: HashMap::new(),
            map: HashMap::new()
        };
    }

    fn add(&mut self, comp: TComponent) -> usize
    {
        let id = self.comps.add(comp);
        self.update_group(id, K::default());
        return id;
    }

    fn remove(&mut self, id: usize)
    {
        if let Some(group) = self.group_map.get(&id) {
            let map = self.map.get_mut(group).unwrap();
            map.retain(|val| *val != id);
            if map.len() == 0 {
                self.map.remove(group);
            }
            self.group_map.remove(&id);
        }
        self.comps.remove(id);
    }

    fn size(&self) -> usize
    {
        return self.comps.size();
    }
}

impl<'a, K: 'a + Sized + Eq + Hash + Copy + Default, TComponent: 'a + Component> IterableComponentPool<'a, TComponent>
    for GroupComponentPool<K, TComponent>
{
    type Iter = GcpIterator<'a, K, TComponent>;
    type IterMut = GcpIteratorMut<'a, K, TComponent>;

    fn iter(&'a self) -> Self::Iter
    {
        return GcpIterator::new(&self.comps, self.map.values());
    }

    fn iter_mut(&'a mut self) -> Self::IterMut
    {
        return GcpIteratorMut::new(&mut self.comps, self.map.values());
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, TComponent: Component> Index<usize> for GroupComponentPool<K, TComponent>
{
    type Output = TComponent;

    fn index(&self, index: usize) -> &Self::Output
    {
        return self.comps.index(index);
    }
}

impl<K: Sized + Eq + Hash + Copy + Default, TComponent: Component> IndexMut<usize> for GroupComponentPool<K, TComponent>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        return self.comps.index_mut(index);
    }
}

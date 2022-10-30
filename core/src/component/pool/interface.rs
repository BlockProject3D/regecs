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

use crate::component::{Component, ComponentRef};
use crate::object::ObjectRef;
use std::ops::{Index, IndexMut};

/// Represents an allocation pool for a given type of component
///
/// *The ComponentPool is a trait to allow customizing the data structure used to store components*
pub trait ComponentPool<T: Component>:
    Index<ComponentRef<T>, Output = T> + IndexMut<ComponentRef<T>>
where
    Self: Sized,
{
    /// Stores a new component in this pool
    ///
    /// # Arguments
    ///
    /// * `comp` - the component to store
    ///
    /// # Returns
    ///
    /// * a reference to the new stored component
    fn add(&mut self, comp: T) -> ComponentRef<T>;

    /// Removes a component from this pool
    ///
    /// # Arguments
    ///
    /// * `r` - a reference to the component to remove
    fn remove(&mut self, r: ComponentRef<T>);

    /// Returns the number of components stored in this pool
    ///
    /// # Returns
    ///
    /// * the component count
    fn len(&self) -> usize;

    /// Returns true if this component pool is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Allows a component pool to be iterated
///
/// *All iterators in component pools returns indices of components*
/// *to get the actual component instance use index or index_mut*
pub trait Iter<'a, T: 'a + Component> {
    /// The type of immutable iterator
    type Iter: Iterator<Item = (ComponentRef<T>, &'a T)>;

    /// The type of mutable iterator
    type IterMut: Iterator<Item = (ComponentRef<T>, &'a mut T)>;

    /// Returns an iterator into this pool
    ///
    /// # Returns
    ///
    /// * a new immutable iterator instance
    fn iter(&'a self) -> Self::Iter;

    /// Returns an iterator into this pool
    ///
    /// # Returns
    ///
    /// * a new mutable iterator instance
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

pub trait Attachments<T: Component> {
    /// Attach a new component
    ///
    /// # Arguments
    ///
    /// * `entity` - the entity to attach the component to
    /// * `component` - the component index to attach
    fn attach(&mut self, entity: ObjectRef, r: ComponentRef<T>);

    /// Lists all attachments of a given entity
    ///
    /// # Arguments
    ///
    /// * `entity` - the entity to list
    ///
    /// # Returns
    ///
    /// * the list of all components attached to the given entity
    /// * None if the entity does not exist or that the entity does not have any attachements
    fn list(&self, entity: ObjectRef) -> Option<Vec<ComponentRef<T>>>;

    /// Removes all components attached to a given entity
    ///
    /// # Arguments
    ///
    /// * `entity` - the entity to clear
    fn clear(&mut self, entity: ObjectRef);

    fn get_first_mut(&mut self, entity: ObjectRef) -> Option<&mut T>;

    fn get_first(&self, entity: ObjectRef) -> Option<&T>;
}

pub trait ComponentManager<T: Component> {
    fn get(&self) -> &T::Pool;
    fn get_mut(&mut self) -> &mut T::Pool;

    fn get_component(&self, r: ComponentRef<T>) -> &T {
        &self.get()[r]
    }

    fn get_component_mut(&mut self, r: ComponentRef<T>) -> &mut T {
        &mut self.get_mut()[r]
    }

    fn add_component(&mut self, comp: T) -> ComponentRef<T> {
        self.get_mut().add(comp)
    }

    fn remove_component(&mut self, r: ComponentRef<T>) {
        self.get_mut().remove(r);
    }
}

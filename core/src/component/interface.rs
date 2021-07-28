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

//! REGECS component interfaces

use std::ops::{Index, IndexMut};

use crate::object::ObjectRef;

/// Represents a component
pub trait Component: Sized
{
    /// The type of ComponentPool to use for storing instances of this component
    type Pool: ComponentPool<Self>;
}

/// Represents an allocation pool for a given type of component
///
/// *The ComponentPool is a trait to allow customizing the data structure used to store components*
pub trait ComponentPool<TComponent: Component>: Index<usize, Output = TComponent> + IndexMut<usize>
{
    /// Creates a new instance of this ComponentPool
    ///
    /// # Returns
    ///
    /// * the new component pool
    fn new() -> Self;

    /// Stores a new component in this pool
    ///
    /// # Arguments
    ///
    /// * `comp` - the component to store
    ///
    /// # Returns
    ///
    /// * the unique index of the new stored component
    fn add(&mut self, comp: TComponent) -> usize;

    /// Removes a component from this pool
    ///
    /// # Arguments
    ///
    /// * `id` - the index of the component to remove
    fn remove(&mut self, id: usize);

    /// Returns the number of components stored in this pool
    ///
    /// # Returns
    ///
    /// * the component count
    fn size(&self) -> usize;
}

/// Allows a component pool to be iterated
///
/// *All iterators in component pools returns indices of components*
/// *to get the actual component instance use index or index_mut*
pub trait IterableComponentPool<'a, TComponent: 'a + Component>
{
    /// The type of immutable iterator
    type Iter: Iterator<Item = &'a TComponent>;

    /// The type of mutable iterator
    type IterMut: Iterator<Item = &'a mut TComponent>;

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

pub trait ComponentProvider<TComponent: Component>
{
    fn pool(&self) -> &TComponent::Pool;
    fn pool_mut(&mut self) -> &mut TComponent::Pool;
}

/// Base trait to represent the container of all component pools
pub trait ComponentManager
{
    /// Clears all components attached to the given entity
    fn clear_components(&mut self, target: ObjectRef);
}

pub fn add_component<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &mut TComponentManager,
    comp: TComponent
) -> usize
{
    return mgr.pool_mut().add(comp);
}

pub fn get_component<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &TComponentManager,
    id: usize
) -> &TComponent
{
    return &mgr.pool()[id];
}

pub fn get_component_mut<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &mut TComponentManager,
    id: usize
) -> &mut TComponent
{
    return &mut mgr.pool_mut()[id];
}

pub fn remove_component<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &mut TComponentManager,
    id: usize
)
{
    mgr.pool_mut().remove(id);
}

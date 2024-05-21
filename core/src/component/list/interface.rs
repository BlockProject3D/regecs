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

use std::ops::{Index, IndexMut};
use crate::component::Component;

/// Represents an allocation list for a given type of component.
///
/// *The [List] is a trait to allow customizing the data structure used to store components.*
pub trait List<T: Component>:
    Index<usize, Output = T> + IndexMut<usize>
where
    Self: Sized,
{
    /// Stores a new component in this list
    ///
    /// # Arguments
    ///
    /// * `comp` - the component to store
    ///
    /// # Returns
    ///
    /// * a reference to the new stored component
    fn add(&mut self, comp: T) -> usize;

    /// Removes a component from this list
    ///
    /// # Arguments
    ///
    /// * `r` - a reference to the component to remove
    fn remove(&mut self, r: usize);

    /// Returns the number of components stored in this list
    ///
    /// # Returns
    ///
    /// * the component count
    fn len(&self) -> usize;

    /// Returns true if this component list is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Allows a component list to be iterated
///
/// *All iterators in component lists returns indices of components*
/// *to get the actual component instance use index or index_mut*
pub trait Iter<'a, T: 'a + Component> {
    /// The type of immutable iterator
    type Iter: Iterator<Item = (usize, &'a T)>;

    /// The type of mutable iterator
    type IterMut: Iterator<Item = (usize, &'a mut T)>;

    /// Returns an iterator into this list
    ///
    /// # Returns
    ///
    /// * a new immutable iterator instance
    fn iter(&'a self) -> Self::Iter;

    /// Returns an iterator into this list
    ///
    /// # Returns
    ///
    /// * a new mutable iterator instance
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

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

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::component::pool::ComponentPool;

use crate::object::ObjectRef;

/// Represents a component
pub trait Component: Sized
{
    /// The type of ComponentPool to use for storing instances of this component
    type Pool: ComponentPool<Self>;
}

pub struct ComponentRef<T: Component>
{
    pub index: usize,
    useless: std::marker::PhantomData<T>
}

impl<T: Component> Copy for ComponentRef<T> {}

impl<T: Component> Clone for ComponentRef<T>
{
    fn clone(&self) -> Self
    {
        Self {
            index: self.index,
            useless: self.useless
        }
    }
}

impl<T: Component> PartialEq for ComponentRef<T>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.index == other.index
    }
}

impl<T: Component> Eq for ComponentRef<T> { }

impl<T: Component> Hash for ComponentRef<T>
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.index.hash(state)
    }
}

impl<T: Component> Debug for ComponentRef<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        <usize as Debug>::fmt(&self.index, f)
    }
}

impl<T: Component> Display for ComponentRef<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        <usize as Display>::fmt(&self.index, f)
    }
}

impl<T: Component> ComponentRef<T>
{
    pub fn new(index: usize) -> Self
    {
        Self {
            index,
            useless: Default::default()
        }
    }
}

pub trait Clear
{
    /// Clears all components attached to the given entity
    fn clear(&mut self, entity: ObjectRef);
}

pub type Pool<T> = <T as Component>::Pool;

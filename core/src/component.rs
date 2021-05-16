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

//! REGECS component layer

use std::vec::Vec;

use crate::object::ObjectRef;

/// Represents an allocation pool for a given type of component
pub struct ComponentPool<TComponent: Sized>
{
    comps: Vec<TComponent>
}

impl <TComponent: Sized> ComponentPool<TComponent>
{
    pub fn new() -> ComponentPool<TComponent>
    {
        return ComponentPool
        {
            comps: Vec::new()
        };
    }

    pub fn add(&mut self, comp: TComponent) -> usize
    {
        let id = self.comps.len();
        self.comps.push(comp);
        return id;
    }

    pub fn get(&mut self, id: usize) -> &mut TComponent
    {
        return &mut self.comps[id];
    }

    pub fn remove(&mut self, id: usize)
    {
        self.comps.remove(id);
    }

    pub fn size(&self) -> usize
    {
        return self.comps.len();
    }
}

pub trait ComponentProvider<TComponent: Sized>
{
    fn get(&mut self, id: usize) -> &mut TComponent;
    fn get_pool(&mut self) -> &mut ComponentPool<TComponent>;
}

/// Base trait to represent the container of all component pools
pub trait ComponentManager
{
    /// Clears all components attached to the given entity
    fn clear_components(&mut self, target: ObjectRef);
}

pub fn add_component<'a, TComponentManager: ComponentProvider<TComponent>, TComponent: Sized>(mgr: &'a mut TComponentManager, comp: TComponent) -> usize
{
    return mgr.get_pool().add(comp);
}

pub fn get_component<'a, TComponentManager: ComponentProvider<TComponent>, TComponent: Sized>(mgr: &'a mut TComponentManager, id: usize) -> &'a mut TComponent
{
    return mgr.get(id);
}

pub fn remove_component<'a, TComponentManager: ComponentProvider<TComponent>, TComponent: Sized>(mgr: &'a mut TComponentManager, id: usize)
{
    mgr.get_pool().remove(id);
}
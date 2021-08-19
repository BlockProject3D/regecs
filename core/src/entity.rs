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

use crate::{
    component::interface::{AttachmentProvider, Component, ComponentPool, ComponentProvider},
    object::ObjectRef
};

pub struct Entity<'a, TComponentManager>
{
    mgr: &'a mut TComponentManager,
    entity: ObjectRef
}

impl<'a, TComponentManager> Entity<'a, TComponentManager>
{
    pub fn aquire(&mut self, other: ObjectRef) -> Entity<TComponentManager>
    {
        return Entity {
            mgr: self.mgr,
            entity: other
        };
    }
}

pub trait EntityPart<TComponent: Component, TComponentManager: ComponentProvider<TComponent>>
{
    fn add(&mut self, comp: TComponent) -> usize;
    fn get_mut(&mut self, _: ComponentType<TComponent>, id: usize) -> &mut TComponent;
    fn get(&self, _: ComponentType<TComponent>, id: usize) -> &TComponent;
    fn remove(&mut self, _: ComponentType<TComponent>, id: usize);
    fn list(&self, _: ComponentType<TComponent>) -> Option<Vec<usize>>;
}

impl<'a, TComponent: Component, TComponentManager: ComponentProvider<TComponent>>
    EntityPart<TComponent, TComponentManager> for Entity<'a, TComponentManager>
where
    TComponent::Pool: AttachmentProvider
{
    fn add(&mut self, comp: TComponent) -> usize
    {
        let id = self.mgr.pool_mut().add(comp);
        self.mgr.pool_mut().attach(self.entity, id);
        return id;
    }

    fn get_mut(&mut self, _: ComponentType<TComponent>, id: usize) -> &mut TComponent
    {
        return &mut self.mgr.pool_mut()[id];
    }

    fn get(&self, _: ComponentType<TComponent>, id: usize) -> &TComponent
    {
        return &self.mgr.pool()[id];
    }

    fn remove(&mut self, _: ComponentType<TComponent>, id: usize)
    {
        self.mgr.pool_mut().remove(id);
    }

    fn list(&self, _: ComponentType<TComponent>) -> Option<Vec<usize>>
    {
        return self.mgr.pool().list(self.entity);
    }
}

pub struct ComponentType<TComponent: Component>
{
    useless: std::marker::PhantomData<TComponent>
}

impl<TComponent: Component> ComponentType<TComponent>
{
    pub fn new() -> ComponentType<TComponent>
    {
        return ComponentType {
            useless: std::marker::PhantomData::default()
        };
    }
}

pub trait ComponentTypeProvider<TComponent: Component>
{
    fn ctype() -> ComponentType<TComponent>;
}

impl<TComponent: Component> ComponentTypeProvider<TComponent> for TComponent
{
    fn ctype() -> ComponentType<TComponent>
    {
        return ComponentType::<TComponent>::new();
    }
}

impl<'a, TComponentManager> Entity<'a, TComponentManager>
{
    pub fn new(mgr: &'a mut TComponentManager, entity: ObjectRef) -> Entity<'a, TComponentManager>
    {
        return Entity { mgr, entity };
    }
}

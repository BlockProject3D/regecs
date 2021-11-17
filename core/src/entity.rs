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
    component::{Component},
    object::ObjectRef
};
use crate::component::ComponentRef;
use crate::component::pool::{Attachments, ComponentManager};

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
    fn class() -> ComponentType<TComponent>;
}

impl<TComponent: Component> ComponentTypeProvider<TComponent> for TComponent
{
    fn class() -> ComponentType<TComponent>
    {
        return ComponentType::<TComponent>::new();
    }
}

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

pub trait EntityPart<T: Component, Provider: ComponentManager<T>>
{
    fn add(&mut self, comp: T) -> ComponentRef<T>;
    fn get_mut(&mut self, r: ComponentRef<T>) -> &mut T;
    fn get(&self, r: ComponentRef<T>) -> &T;
    fn remove(&mut self, r: ComponentRef<T>);
    fn list(&self, _: ComponentType<T>) -> Option<Vec<ComponentRef<T>>>;
    fn get_first(&self, _: ComponentType<T>) -> Option<&T>;
    fn get_first_mut(&mut self, _: ComponentType<T>) -> Option<&mut T>;
}

impl<'a, T: Component, Provider: ComponentManager<T>>
    EntityPart<T, Provider> for Entity<'a, Provider>
where
    T::Pool: Attachments<T>
{
    fn add(&mut self, comp: T) -> ComponentRef<T>
    {
        let r = self.mgr.add_component(comp);
        self.mgr.get_mut().attach(self.entity, r);
        return r;
    }

    fn get_mut(&mut self, r: ComponentRef<T>) -> &mut T
    {
        self.mgr.get_component_mut(r)
    }

    fn get(&self, r: ComponentRef<T>) -> &T
    {
        self.mgr.get_component(r)
    }

    fn remove(&mut self, r: ComponentRef<T>)
    {
        self.mgr.remove_component(r)
    }

    fn list(&self, _: ComponentType<T>) -> Option<Vec<ComponentRef<T>>>
    {
        return self.mgr.get().list(self.entity);
    }

    fn get_first(&self, _: ComponentType<T>) -> Option<&T>
    {
        self.mgr.get().get_first(self.entity)
    }

    fn get_first_mut(&mut self, _: ComponentType<T>) -> Option<&mut T>
    {
        self.mgr.get_mut().get_first_mut(self.entity)
    }
}

impl<'a, TComponentManager> Entity<'a, TComponentManager>
{
    pub fn new(mgr: &'a mut TComponentManager, entity: ObjectRef) -> Entity<'a, TComponentManager>
    {
        return Entity { mgr, entity };
    }
}

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

use crate::event::EventManager;
use crate::object::{Context, ObjectRef, Tree};
use crate::scene::event::Event;
use crate::scene::{Interface, Notify};
use std::marker::PhantomData;

//TODO: Find better names for fields.

pub struct SystemState<C: Context> {
    pub(crate) component_manager: C::Pool,
    pub(crate) event_manager: EventManager<C::Event>,
    pub(crate) system_event_manager: EventManager<Event<C>>,
    pub(crate) tree: Tree,
}

impl<C: Context> crate::system::Context for SystemState<C> {
    type Builder = C::Builder;
    type AppState = C::AppState;
    type Pool = C::Pool;
    type Event = C::Event;

    fn pool(&self) -> &Self::Pool {
        return &self.component_manager;
    }

    fn pool_mut(&mut self) -> &mut Self::Pool {
        return &mut self.component_manager;
    }

    fn event_manager(&mut self) -> &mut EventManager<Self::Event> {
        return &mut self.event_manager;
    }

    fn objects(&self) -> &Tree {
        return &self.tree;
    }

    fn enable_object(&mut self, notify: Notify, target: ObjectRef, enable: bool) {
        let builder = notify
            .into_builder(super::event::Type::EnableObject(enable))
            .target(target);
        self.system_event_manager.send(builder);
    }

    fn remove_object(&mut self, notify: Notify, target: ObjectRef) {
        let builder = notify
            .into_builder(super::event::Type::RemoveObject)
            .target(target);
        self.system_event_manager.send(builder);
    }

    fn spawn_object(&mut self, notify: Notify, builder: Self::Builder) {
        let builder = notify.into_builder(super::event::Type::SpawnObject(builder));
        self.system_event_manager.send(builder);
    }
}

pub struct ObjectState<I: Interface> {
    pub(crate) common: SystemState<Self>,
    pub(crate) systems: I::SystemManager,
    pub(crate) useless: PhantomData<I::Builder>,
}

impl<I: Interface> crate::system::Context for ObjectState<I> {
    type Builder = I::Builder;
    type AppState = I::AppState;
    type Pool = I::ComponentManager;
    type Event = I::Event;

    fn pool(&self) -> &Self::Pool {
        return &self.common.component_manager;
    }

    fn pool_mut(&mut self) -> &mut Self::Pool {
        return &mut self.common.component_manager;
    }

    fn event_manager(&mut self) -> &mut EventManager<Self::Event> {
        return &mut self.common.event_manager;
    }

    fn objects(&self) -> &Tree {
        return &self.common.tree;
    }

    fn enable_object(&mut self, notify: Notify, target: ObjectRef, enable: bool) {
        self.common.enable_object(notify, target, enable)
    }

    fn remove_object(&mut self, notify: Notify, target: ObjectRef) {
        self.common.remove_object(notify, target)
    }

    fn spawn_object(&mut self, notify: Notify, builder: Self::Builder) {
        self.common.spawn_object(notify, builder)
    }
}

impl<I: Interface> Context for ObjectState<I> {
    type SystemManager = I::SystemManager;

    fn systems(&self) -> &Self::SystemManager {
        return &self.systems;
    }

    fn systems_mut(&mut self) -> &mut Self::SystemManager {
        return &mut self.systems;
    }
}

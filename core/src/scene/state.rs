// Copyright (c) 2022, BlockProject 3D
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

use std::marker::PhantomData;
use crate::component::Clear;
use crate::event::EventManager;
use crate::object::{Context, Tree, Factory};
use crate::scene::event::{Event};
use crate::scene::EventInfo;

//TODO: Find better names for fields.

pub struct Common<C: Context>
{
    pub(crate) component_manager: C::ComponentManager,
    pub(crate) event_manager: EventManager<C::Event>,
    pub(crate) system_event_manager: EventManager<Event<C>>,
    pub(crate) tree: Tree
}

impl<C: Context> crate::system::Context for Common<C>
{
    type Factory = C::Factory;
    type AppState = C::AppState;
    type ComponentManager = C::ComponentManager;
    type Event = C::Event;

    fn components(&self) -> &Self::ComponentManager {
        return &self.component_manager;
    }

    fn components_mut(&mut self) -> &mut Self::ComponentManager {
        return &mut self.component_manager;
    }

    fn event_manager(&mut self) -> &mut EventManager<Self::Event> {
        return &mut self.event_manager;
    }

    fn objects(&self) -> &Tree {
        return &self.tree;
    }

    fn enable_object(&mut self, info: EventInfo, enable: bool) {
        let ty = super::event::Type::EnableObject(enable);
        self.system_event_manager.send(info.into_event(ty));
    }

    fn remove_object(&mut self, info: EventInfo) {
        let ty = super::event::Type::RemoveObject;
        self.system_event_manager.send(info.into_event(ty));
    }

    fn spawn_object(&mut self, info: EventInfo, factory: Self::Factory) {
        let ty = super::event::Type::SpawnObject(factory);
        self.system_event_manager.send(info.into_event(ty));
    }
}

pub struct State<E, S, CM: Clear, SM, F: Factory<State<E, S, CM, SM, F>>> {
    pub(crate) common: Common<Self>,
    pub(crate) systems: SM,
    pub(crate) useless: PhantomData<F>
}

impl<E, S, CM: Clear, SM, F: Factory<State<E, S, CM, SM, F>>> crate::system::Context for State<E, S, CM, SM, F> {
    type Factory = F;
    type AppState = S;
    type ComponentManager = CM;
    type Event = E;

    fn components(&self) -> &Self::ComponentManager {
        return &self.common.component_manager;
    }

    fn components_mut(&mut self) -> &mut Self::ComponentManager {
        return &mut self.common.component_manager;
    }

    fn event_manager(&mut self) -> &mut EventManager<Self::Event> {
        return &mut self.common.event_manager;
    }

    fn objects(&self) -> &Tree {
        return &self.common.tree;
    }

    fn enable_object(&mut self, info: EventInfo, enable: bool) {
        self.common.enable_object(info, enable)
    }

    fn remove_object(&mut self, info: EventInfo) {
        self.common.remove_object(info)
    }

    fn spawn_object(&mut self, info: EventInfo, factory: Self::Factory) {
        self.common.spawn_object(info, factory)
    }
}

impl<E, S, CM: Clear, SM, F: Factory<State<E, S, CM, SM, F>>> Context for State<E, S, CM, SM, F>
{
    type SystemManager = SM;

    fn systems(&self) -> &Self::SystemManager {
        return &self.systems;
    }

    fn systems_mut(&mut self) -> &mut Self::SystemManager {
        return &mut self.systems;
    }
}

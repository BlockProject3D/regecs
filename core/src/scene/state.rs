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

use crate::component::Clear;
use crate::event::EventManager;
use crate::object::{Context, ObjectTree};
use crate::scene::event::{Event};

pub struct SystemState<C: Context>
{
    pub component_manager: C::ComponentManager,
    pub event_manager: EventManager<C::Event>,
    pub system_event_manager: EventManager<Event<C>>,
    pub tree: ObjectTree
}

impl<C: Context> crate::system::Context for SystemState<C>
{
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

    fn objects(&self) -> &ObjectTree {
        return &self.tree;
    }
}

pub struct ObjectState<E, S, CM: Clear, SM> {
    pub common: SystemState<Self>,
    pub systems: SM
}

impl<E, S, CM: Clear, SM> crate::system::Context for ObjectState<E, S, CM, SM> {
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

    fn objects(&self) -> &ObjectTree {
        return &self.common.tree;
    }
}

impl<E, S, CM: Clear, SM> Context for ObjectState<E, S, CM, SM>
{
    type SystemManager = SM;

    fn systems(&self) -> &Self::SystemManager {
        return &self.systems;
    }

    fn systems_mut(&mut self) -> &mut Self::SystemManager {
        return &mut self.systems;
    }
}

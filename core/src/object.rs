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

//! REGECS object and entity layer

use std::{any::Any, boxed::Box};

use crate::component::interface::ComponentManager;
use crate::event::{EventManager, EventSender};

pub struct EventContext<'a, TState, TComponentManager, TEventSender: EventSender<TState, TComponentManager>>
{
    pub components: &'a mut TComponentManager,
    pub event_manager: &'a TEventSender,
    pub this: ObjectRef,
    pub sender: Option<ObjectRef>,
    pub state: &'a mut TState,
}

pub struct CommonContext<'a, TState, TComponentManager, TEventSender: EventSender<TState, TComponentManager>>
{
    pub components: &'a mut TComponentManager,
    pub event_manager: &'a TEventSender,
    pub this: ObjectRef,
    pub state: &'a mut TState,
}

/// Type alias for object references
///
/// *serves also as entry point into REGECS entity layer*
pub type ObjectRef = u32;

/// Low-level object interface to represent all dynamic objects managed by a scene
pub trait CoreObject<TState, TComponentManager>
{
    fn on_event(
        &mut self,
        context: EventContext<TState, TComponentManager, EventManager<TState, TComponentManager>>,
        event: &Box<dyn Any>,
    ) -> Option<Box<dyn Any>>;
    fn on_init(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>);
    fn on_remove(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>);
    fn on_update(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>);
    fn serialize(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>) -> Option<bpx::sd::Object>;
    fn deserialize(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>, obj: bpx::sd::Object);
}

/// High-level object interface
pub trait Object<TState, TComponentManager>
{
    type EventType: Any;

    fn handle_event<T: Any>(
        &mut self,
        context: EventContext<TState, TComponentManager, EventManager<TState, TComponentManager>>,
        event: &Self::EventType,
    ) -> Option<T>;
    fn init(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>);
    fn remove(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>);
    fn update(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>);
    fn serialize(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>) -> Option<bpx::sd::Object>;
    fn deserialize(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>, obj: bpx::sd::Object);
}

impl<
    TState,
    TComponentManager: ComponentManager,
    EventType: Any,
    O: Object<TState, TComponentManager, EventType=EventType>
> CoreObject<TState, TComponentManager> for O
{
    fn on_event(
        &mut self,
        context: EventContext<TState, TComponentManager, EventManager<TState, TComponentManager>>,
        event: &Box<dyn Any>,
    ) -> Option<Box<dyn Any>>
    {
        if let Some(ev) = event.downcast_ref::<EventType>() {
            return self.handle_event(context, &ev);
        }
        return None;
    }

    fn on_init(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>)
    {
        self.init(context);
    }

    fn on_remove(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>)
    {
        context.components.clear_components(context.this);
        self.remove(context);
    }

    fn on_update(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>)
    {
        self.update(context);
    }

    fn serialize(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>) -> Option<bpx::sd::Object>
    {
        return self.serialize(context);
    }

    fn deserialize(&mut self, context: CommonContext<TState, TComponentManager, EventManager<TState, TComponentManager>>, obj: bpx::sd::Object)
    {
        self.deserialize(context, obj);
    }
}

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

use crate::event::EventManager;
use std::any::Any;
use crate::component::ComponentManager;
use crate::object::ObjectTree;
use std::cell::RefCell;
use crate::system::SystemList;

/// Type alias for object references
///
/// *serves also as entry point into REGECS entity layer*
pub type ObjectRef = u32;

pub trait Context : Sized
{
    type AppState;
    type ComponentManager: ComponentManager;
    type SystemContext: crate::system::Context;
    type SystemList: SystemList<Self::SystemContext>;

    fn components(&self) -> &RefCell<Self::ComponentManager>;
    fn systems(&self) -> &RefCell<Self::SystemList>;
    fn event_manager(&self) -> &RefCell<EventManager<Self>>;
    fn objects(&self) -> &ObjectTree;
}

/// Low-level object interface to represent all dynamic objects managed by a scene
pub trait CoreObject<TContext: Context>
{
    fn on_event(
        &mut self,
        ctx: &TContext,
        state: &TContext::AppState,
        event: &Box<dyn Any>,
        sender: Option<ObjectRef>,
        this: ObjectRef,
    ) -> Option<Box<dyn Any>>;
    fn on_init(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef);
    fn on_remove(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef);
    fn on_update(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef);
    fn serialize(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef) -> Option<bpx::sd::Object>;
    fn deserialize(&mut self, ctx: &TContext, state: &TContext::AppState, obj: bpx::sd::Object, this: ObjectRef);
    fn class(&self) -> &str;
}

/// High-level object interface
pub trait Object<TContext: Context>
{
    type EventType: Any;

    fn handle_event<T: Any>(
        &mut self,
        ctx: &TContext,
        state: &TContext::AppState,
        event: &Self::EventType,
        sender: Option<ObjectRef>,
        this: ObjectRef,
    ) -> Option<T>;
    fn init(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef);
    fn remove(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef);
    fn update(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef);
    fn serialize(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef) -> Option<bpx::sd::Object>;
    fn deserialize(&mut self, ctx: &TContext, state: &TContext::AppState, obj: bpx::sd::Object, this: ObjectRef);
}

impl<
    TContext: Context,
    EventType: Any,
    O: Object<TContext, EventType=EventType>
> CoreObject<TContext> for O
{
    fn on_event(
        &mut self,
        ctx: &TContext,
        state: &TContext::AppState,
        event: &Box<dyn Any>,
        sender: Option<ObjectRef>,
        this: ObjectRef,
    ) -> Option<Box<dyn Any>>
    {
        if let Some(ev) = event.downcast_ref::<EventType>() {
            return self.handle_event(ctx, state, &ev, sender, this);
        }
        return None;
    }

    fn on_init(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef)
    {
        self.init(ctx, state, this);
    }

    fn on_remove(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef)
    {
        ctx.components().borrow_mut().clear_components(this);
        self.remove(ctx, state, this);
    }

    fn on_update(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef)
    {
        self.update(ctx, state, this);
    }

    fn serialize(&mut self, ctx: &TContext, state: &TContext::AppState, this: ObjectRef) -> Option<bpx::sd::Object>
    {
        return self.serialize(ctx, state, this);
    }

    fn deserialize(&mut self, ctx: &TContext, state: &TContext::AppState, obj: bpx::sd::Object, this: ObjectRef)
    {
        self.deserialize(ctx, state, obj, this);
    }

    fn class(&self) -> &str
    {
        //Not yet connected to reflection system
        return "Generic";
    }
}

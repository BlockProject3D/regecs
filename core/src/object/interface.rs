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

use std::any::Any;

use crate::{
    component::ComponentManager,
    event::EventManager,
    object::ObjectTree,
    system::SystemManager
};

/// Type alias for object references
///
/// *serves also as entry point into REGECS entity layer*
pub type ObjectRef = u32;

pub trait Context: Sized
{
    type AppState;
    type ComponentManager: ComponentManager;
    type SystemContext: crate::system::Context;
    type SystemManager: SystemManager<Self::SystemContext>;

    fn components(&self) -> &Self::ComponentManager;
    fn components_mut(&mut self) -> &mut Self::ComponentManager;
    fn event_manager(&mut self) -> &mut EventManager<Self>;
    fn systems(&self) -> &Self::SystemManager;
    fn systems_mut(&mut self) -> &mut Self::SystemManager;
    fn objects(&self) -> &ObjectTree;
}

pub trait Serializable<TContext: Context>
{
    fn serialize(&self, ctx: &TContext, state: &TContext::AppState) -> Option<bpx::sd::Object>;
    fn deserialize(&mut self, ctx: &mut TContext, state: &TContext::AppState, obj: bpx::sd::Object);
}

pub trait Index
{
    fn index(&self) -> ObjectRef;
}

/// Low-level object interface to represent all dynamic objects managed by a scene
pub trait CoreObject<TContext: Context>: Serializable<TContext>
{
    fn on_event(
        &mut self,
        ctx: &mut TContext,
        state: &TContext::AppState,
        event: &Box<dyn Any>,
        sender: Option<ObjectRef>
    ) -> Option<Box<dyn Any>>;
    /// Return true to enable updates on this object
    fn on_init(&mut self, ctx: &mut TContext, state: &TContext::AppState) -> bool;
    fn on_remove(&mut self, ctx: &mut TContext, state: &TContext::AppState);
    fn on_update(&mut self, ctx: &mut TContext, state: &TContext::AppState);
    fn class(&self) -> &str;
}

/// High-level object interface
pub trait Object<TContext: Context>
{
    type EventType: Any;

    fn handle_event<T: Any>(
        &mut self,
        ctx: &mut TContext,
        state: &TContext::AppState,
        event: &Self::EventType,
        sender: Option<ObjectRef>
    ) -> Option<T>;
    /// Return true to enable updates on this object
    fn init(&mut self, ctx: &mut TContext, state: &TContext::AppState) -> bool;
    fn remove(&mut self, ctx: &mut TContext, state: &TContext::AppState);
    fn update(&mut self, ctx: &mut TContext, state: &TContext::AppState);
}

impl<
        TContext: Context,
        EventType: Any,
        O: Object<TContext, EventType = EventType> + Serializable<TContext> + Index
    > CoreObject<TContext> for O
{
    fn on_event(
        &mut self,
        ctx: &mut TContext,
        state: &TContext::AppState,
        event: &Box<dyn Any>,
        sender: Option<ObjectRef>
    ) -> Option<Box<dyn Any>>
    {
        if let Some(ev) = event.downcast_ref::<EventType>() {
            return self.handle_event(ctx, state, ev, sender);
        }
        return None;
    }

    fn on_init(&mut self, ctx: &mut TContext, state: &TContext::AppState) -> bool
    {
        return self.init(ctx, state);
    }

    fn on_remove(&mut self, ctx: &mut TContext, state: &TContext::AppState)
    {
        ctx.components_mut().clear_components(self.index());
        self.remove(ctx, state);
    }

    fn on_update(&mut self, ctx: &mut TContext, state: &TContext::AppState)
    {
        self.update(ctx, state);
    }

    fn class(&self) -> &str
    {
        //Not yet connected to reflection system
        return "Generic";
    }
}

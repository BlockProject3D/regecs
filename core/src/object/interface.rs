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
    event::EventManager,
    object::ObjectTree
};
use crate::component::Clear;

/// Type alias for object references
///
/// *serves also as entry point into REGECS entity layer*
pub type ObjectRef = u32;

pub trait Context: Sized
{
    type Event;
    type AppState;
    type ComponentManager: Clear;
    type SystemManager;

    fn components(&self) -> &Self::ComponentManager;
    fn components_mut(&mut self) -> &mut Self::ComponentManager;
    fn event_manager(&mut self) -> &mut EventManager<Self>;
    fn systems(&self) -> &Self::SystemManager;
    fn systems_mut(&mut self) -> &mut Self::SystemManager;
    fn objects(&self) -> &ObjectTree;
}

pub trait Index
{
    fn index(&self) -> ObjectRef;
}

/// Low-level object interface to represent all dynamic objects managed by a scene
pub trait Object<C: Context>
{
    fn on_event(
        &mut self,
        ctx: &mut C,
        state: &C::AppState,
        event: &C::Event,
        sender: Option<ObjectRef>
    );
    /// Return true to enable updates on this object
    fn on_init(&mut self, ctx: &mut C, state: &C::AppState) -> bool;
    fn on_remove(&mut self, ctx: &mut C, state: &C::AppState);
    fn on_update(&mut self, ctx: &mut C, state: &C::AppState);
    fn class(&self) -> &str;
}

/*/// High-level object interface
pub trait Object<C: Context>
{
    type EventType: Any;

    fn handle_event<R: Any>(
        &mut self,
        ctx: &mut C,
        state: &C::AppState,
        event: &Self::EventType,
        sender: Option<ObjectRef>
    ) -> Option<R>;
    /// Return true to enable updates on this object
    fn init(&mut self, ctx: &mut C, state: &C::AppState) -> bool;
    fn remove(&mut self, ctx: &mut C, state: &C::AppState);
    fn update(&mut self, ctx: &mut C, state: &C::AppState);
}

impl<
        C: Context,
        E: Any,
        O: Object<C, EventType = E> + Index
    > CoreObject<C> for O
{
    fn on_event(
        &mut self,
        ctx: &mut C,
        state: &C::AppState,
        event: &Box<dyn Any>,
        sender: Option<ObjectRef>
    ) -> Option<Box<dyn Any>>
    {
        if let Some(ev) = event.downcast_ref::<E>() {
            return self.handle_event(ctx, state, ev, sender);
        }
        return None;
    }

    fn on_init(&mut self, ctx: &mut C, state: &C::AppState) -> bool
    {
        return self.init(ctx, state);
    }

    fn on_remove(&mut self, ctx: &mut C, state: &C::AppState)
    {
        ctx.components_mut().clear(self.index());
        self.remove(ctx, state);
    }

    fn on_update(&mut self, ctx: &mut C, state: &C::AppState)
    {
        self.update(ctx, state);
    }

    fn class(&self) -> &str
    {
        //Not yet connected to reflection system
        return "Generic";
    }
}*/

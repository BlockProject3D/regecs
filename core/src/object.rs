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

use std::boxed::Box;
use std::any::Any;

use crate::event::EventContext;
use crate::event::EventResult;

/// Type alias for object references
/// 
/// *serves also as entry point into REGECS entity layer*
pub type ObjectRef = u32;

/// Low-level object interface to represent all dynamic objects managed by a scene
pub trait LowObject<TState, TComponentManager>
{
    fn on_event(&mut self, event: Box<dyn Any>, context: EventContext<TState, TComponentManager>) -> Option<EventResult>;
    fn on_init(&mut self, ptr: ObjectRef, components: &mut TComponentManager);
    fn on_remove(&mut self, ptr: ObjectRef, components: &mut TComponentManager);
}

/// High-level object interface
pub trait Object<TState, TComponentManager>
{
    type EventType: Any;

    fn event(&mut self, event: &Self::EventType, context: EventContext<TState, TComponentManager>) -> Option<EventResult>;
    fn init(&mut self, ptr: ObjectRef, components: &mut TComponentManager);
    fn remove(&mut self, ptr: ObjectRef, components: &mut TComponentManager);
}

impl <TState, TComponentManager, EventType: Any, O: Object<TState, TComponentManager, EventType = EventType>> LowObject<TState, TComponentManager> for O
{
    fn on_event(&mut self, event: Box<dyn Any>, context: EventContext<TState, TComponentManager>) -> Option<EventResult>
    {
        if let Ok(ev) = event.downcast::<EventType>()
        {
            return self.event(&ev, context);
        }
        return None;
    }

    fn on_init(&mut self, ptr: ObjectRef, components: &mut TComponentManager)
    {
        self.init(ptr, components);
    }

    fn on_remove(&mut self, ptr: ObjectRef, components: &mut TComponentManager)
    {
        self.remove(ptr, components);
    }
}

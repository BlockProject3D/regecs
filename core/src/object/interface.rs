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

use crate::event::{Builder, Event};
use crate::scene::Notify;
use std::num::NonZeroU32;

/// Type alias for object references
///
/// *serves also as entry point into REGECS entity layer*
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct ObjectRef(NonZeroU32);

impl ObjectRef {
    /// Creates a new ObjectRef from a raw u32 index.
    ///
    /// # Arguments
    ///
    /// * `raw`: the raw u32 index.
    ///
    /// returns: ObjectRef
    ///
    /// # Safety
    ///
    /// This function assumes the raw index actually points to an object in the scene, if not
    /// then the behavior when using such dangling reference is undefined.
    /// *Note: it is forbidden to allocate an ObjectRef of 0.*
    pub unsafe fn from_raw(raw: u32) -> ObjectRef {
        ObjectRef(NonZeroU32::new_unchecked(raw))
    }

    pub fn into_raw(self) -> u32 {
        self.0.get()
    }

    pub fn send<C: Context>(&self, ctx: &mut C, sender: Option<ObjectRef>, event: C::Event) {
        let mut builder = Builder::new(event).target(*self);
        if let Some(sender) = sender {
            builder = builder.sender(sender);
        }
        ctx.event_manager().send(builder);
    }

    pub fn enable<C: Context>(&self, ctx: &mut C, notify: Notify, enable: bool) {
        ctx.enable_object(notify, *self, enable);
    }

    pub fn remove<C: Context>(&self, ctx: &mut C, notify: Notify) {
        ctx.remove_object(notify, *self);
    }
}

pub trait Context: crate::system::Context {
    type SystemManager;

    fn systems(&self) -> &Self::SystemManager;
    fn systems_mut(&mut self) -> &mut Self::SystemManager;
}

pub trait Index {
    fn index(&self) -> ObjectRef;
}

pub struct Flags {
    updates: bool,
    receives_events: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            updates: false,
            receives_events: false,
        }
    }

    pub fn is_updatable(&self) -> bool {
        self.updates
    }

    pub fn is_event_aware(&self) -> bool {
        self.receives_events
    }

    pub fn updates(mut self, updates: bool) -> Self {
        self.updates = updates;
        self
    }

    pub fn receives_events(mut self, receives_events: bool) -> Self {
        self.receives_events = receives_events;
        self
    }
}

pub trait Class {
    fn class(&self) -> &str;
}

/// Object interface to represent all objects managed by a scene
pub trait Object<C: Context>: Class {
    fn on_event(&mut self, ctx: &mut C, state: &C::AppState, event: &Event<C::Event>);
    fn on_remove(&mut self, ctx: &mut C, state: &C::AppState);
    fn on_update(&mut self, ctx: &mut C, state: &C::AppState);

    fn flags(&self) -> Flags {
        Flags::new()
    }
}

pub trait New<C: Context> {
    type Arguments;

    fn new(ctx: &mut C, state: &C::AppState, this: ObjectRef, args: Self::Arguments) -> Self;
}

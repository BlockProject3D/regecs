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

use crate::event::Builder;
use crate::object::{Context, Factory, ObjectRef};

pub enum Type<C: Context> {
    EnableObject(bool),
    RemoveObject,
    SpawnObject(Factory<C>)
}

pub struct Event<C: Context> {
    pub notify: bool,
    pub ty: Type<C>
}

pub struct EventInfo {
    sender: Option<ObjectRef>,
    target: Option<ObjectRef>,
    notify: bool
}

impl EventInfo {
    pub fn new() -> Self {
        EventInfo {
            sender: None,
            target: None,
            notify: false
        }
    }

    pub fn sender(mut self, sender: ObjectRef) -> Self {
        self.sender = Some(sender);
        self
    }

    pub fn target(mut self, target: ObjectRef) -> Self {
        self.target = Some(target);
        self
    }

    pub fn notify(mut self) -> Self {
        self.notify = true;
        self
    }

    pub(crate) fn into_event<C: Context>(self, ty: Type<C>) -> Builder<Event<C>> {
        let ev = Event {
            notify: self.notify,
            ty
        };
        let mut builder = Builder::new(ev);
        if let Some(sender) = self.sender {
            builder = builder.sender(sender);
        }
        if let Some(target) = self.target {
            builder = builder.sender(target);
        }
        builder
    }
}
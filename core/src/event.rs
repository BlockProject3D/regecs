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

//! REGECS event system.

use std::collections::VecDeque;
use std::ops::Deref;

use crate::object::ObjectRef;

#[derive(Clone)]
pub struct Event<E> {
    sender: Option<ObjectRef>,
    target: Option<ObjectRef>,
    data: E,
}

impl<E> Event<E> {
    pub fn sender(&self) -> Option<ObjectRef> {
        self.sender
    }

    pub fn target(&self) -> Option<ObjectRef> {
        self.target
    }

    pub fn into_inner(self) -> E {
        self.data
    }
}

impl<E> Deref for Event<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct Builder<E>(Event<E>);

impl<E> Builder<E> {
    pub fn new(data: E) -> Self {
        Self(Event {
            data,
            sender: None,
            target: None,
        })
    }

    pub fn sender(mut self, sender: ObjectRef) -> Self {
        self.0.sender = Some(sender);
        self
    }

    pub fn target(mut self, target: ObjectRef) -> Self {
        self.0.target = Some(target);
        self
    }

    pub fn into_inner(self) -> Event<E> {
        self.0
    }
}

//TODO: Rewrite EventManager into a generic EventManager for any Event type. In scene declare 2
// event managers: a system event manager (to only manage system events) and an object event
// manager (which only manages events of Context::Event type, common to all objects).

pub struct EventManager<E> {
    events: VecDeque<Event<E>>,
}

impl<E> EventManager<E> {
    pub fn new() -> EventManager<E> {
        EventManager {
            events: VecDeque::new(),
        }
    }

    pub fn send(&mut self, event: Builder<E>) {
        self.events.push_back(event.into_inner());
    }

    pub fn poll(&mut self) -> Option<Event<E>> {
        return self.events.pop_front();
    }
}

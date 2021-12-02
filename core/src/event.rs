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

//! REGECS event system

use std::{
    boxed::Box,
    collections::{HashMap, VecDeque}
};

use crate::object::{Context, ObjectFactory, ObjectRef};

pub struct Event<E>
{
    pub sender: Option<ObjectRef>,
    pub target: Option<ObjectRef>,
    pub data: E,
}

pub struct EventBuilder<E>
{
    ev: Event<E>
}

impl<E> EventBuilder<E>
{
    pub fn new(event: E) -> EventBuilder<E>
    {
        return EventBuilder {
            ev: Event {
                sender: None,
                target: None,
                data: event
            }
        };
    }

    pub fn sender(mut self, this: ObjectRef) -> Self
    {
        self.ev.sender = Some(this);
        return self;
    }

    pub fn target(mut self, target: ObjectRef) -> Self
    {
        self.ev.target = Some(target);
        return self;
    }

    pub fn into(self) -> Event<E>
    {
        return self.ev;
    }
}

pub enum SystemEvent<C: Context>
{
    Enable(ObjectRef, bool),
    Spawn(ObjectFactory<C>),
    Destroy(ObjectRef)
}

pub struct EventManager<C: Context>
{
    events: VecDeque<Event<C::Event>>,
    system_events: VecDeque<(bool, SystemEvent<C>)>
}

impl<C: Context> EventManager<C>
{
    pub fn new() -> EventManager<C>
    {
        return EventManager {
            events: VecDeque::new(),
            system_events: VecDeque::new()
        };
    }

    pub fn send(&mut self, event: EventBuilder<C::Event>)
    {
        self.events.push_back(event.into());
    }

    pub fn system(&mut self, event: SystemEvent<C>, notify: bool)
    {
        self.system_events.push_back((notify, event));
    }

    pub fn poll_event(&mut self) -> Option<Event<C::Event>>
    {
        return self.events.pop_front();
    }

    pub fn poll_system_event(&mut self) -> Option<(bool, SystemEvent<C>)>
    {
        return self.system_events.pop_front();
    }
}

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
    any::Any,
    boxed::Box,
    collections::{HashMap, VecDeque}
};

use crate::object::{Context, ObjectFactory, ObjectRef};

pub type Handle = usize;

type EventTrackerValue = Option<Box<dyn Any>>;
type EventTrackerFunc<T, C, State> = Box<dyn FnOnce(&mut T, &mut C, &State, EventTrackerValue)>;

pub struct EventTracker<T, C, State>
{
    events: Vec<(Handle, EventTrackerFunc<T, C, State>)>
}

impl<T, C, State> EventTracker<T, C, State>
{
    pub fn new() -> EventTracker<T, C, State>
    {
        return EventTracker { events: Vec::new() };
    }

    pub fn push<R: 'static, F: 'static + FnOnce(&mut T, &mut C, &State, Option<R>)>(
        &mut self,
        handle: Handle,
        func: F
    )
    {
        self.events.push((
            handle,
            Box::new(|me, ctx, state, data| {
                if let Some(obj) = data {
                    let o = *obj.downcast().unwrap();
                    func(me, ctx, state, o);
                } else {
                    func(me, ctx, state, None);
                }
            })
        ));
    }

    pub fn poll_batch<EventContext: Context>(
        &mut self,
        event_manager: &mut EventManager<EventContext>
    ) -> EventTrackerBatch<T, C, State>
    {
        let mut batch = Vec::new();
        let mut i = 0;
        while i < self.events.len() {
            let (flag, data) = event_manager.track_event(self.events[i].0);
            if flag {
                let (_, func) = self.events.remove(i);
                batch.push((data, func));
            } else {
                i += 1;
            }
        }
        return EventTrackerBatch { events: batch };
    }
}

pub struct EventTrackerBatch<T, C, State>
{
    events: Vec<(EventTrackerValue, EventTrackerFunc<T, C, State>)>
}

impl<T, C, State> EventTrackerBatch<T, C, State>
{
    pub fn run(self, me: &mut T, ctx: &mut C, state: &State)
    {
        for (data, func) in self.events {
            func(me, ctx, state, data);
        }
    }
}

pub struct Event
{
    pub sender: Option<ObjectRef>,
    pub target: Option<ObjectRef>,
    pub data: Box<dyn Any>,
    pub tracking: bool,
    pub handle: Handle
}

pub struct EventBuilder
{
    ev: Event
}

impl EventBuilder
{
    pub fn new<E: Any>(event: E) -> EventBuilder
    {
        return EventBuilder {
            ev: Event {
                sender: None,
                target: None,
                data: Box::from(event),
                tracking: false,
                handle: 0
            }
        };
    }

    pub fn with_sender(mut self, this: ObjectRef) -> EventBuilder
    {
        self.ev.sender = Some(this);
        return self;
    }

    pub fn with_target(mut self, target: ObjectRef) -> EventBuilder
    {
        self.ev.target = Some(target);
        return self;
    }

    pub fn with_tracking(mut self) -> EventBuilder
    {
        self.ev.tracking = true;
        return self;
    }

    pub fn into(self) -> Event
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
    events: VecDeque<Event>,
    system_events: VecDeque<(bool, Handle, SystemEvent<C>)>,
    cur_handle: Handle,
    event_responses: HashMap<Handle, Option<Box<dyn Any>>>
}

impl<C: Context> EventManager<C>
{
    pub fn new() -> EventManager<C>
    {
        return EventManager {
            events: VecDeque::new(),
            system_events: VecDeque::new(),
            cur_handle: 0,
            event_responses: HashMap::new()
        };
    }

    pub fn send(&mut self, event: EventBuilder) -> Handle
    {
        let handle = self.cur_handle;
        let mut e = event.into();
        e.handle = handle;
        self.cur_handle += 1;
        self.events.push_back(e);
        return handle;
    }

    pub fn system(&mut self, event: SystemEvent<C>, tracking: bool) -> Handle
    {
        let handle = self.cur_handle;
        self.cur_handle += 1;
        self.system_events.push_back((tracking, handle, event));
        return handle;
    }

    pub fn track_event(&mut self, handle: Handle) -> (bool, Option<Box<dyn Any>>)
    {
        if let Some(data) = self.event_responses.remove(&handle) {
            return (true, data);
        }
        return (false, None);
    }

    pub fn poll_event(&mut self) -> Option<Event>
    {
        return self.events.pop_front();
    }

    pub fn poll_system_event(&mut self) -> Option<(bool, Handle, SystemEvent<C>)>
    {
        return self.system_events.pop_front();
    }

    pub fn queue_response(&mut self, handle: Handle, response: Option<Box<dyn Any>>)
    {
        self.event_responses.insert(handle, response);
    }
}

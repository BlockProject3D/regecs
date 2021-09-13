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

use std::{any::Any, boxed::Box};

use crate::object::{ObjectRef, CoreObject, CommonContext};
use std::collections::{HashMap, VecDeque};

pub type Handle = usize;

pub struct EventTracker<T, TState, TComponentManager>
{
    events: Vec<(Handle, Box<dyn Fn(&mut T, &mut CommonContext<TState, TComponentManager>, Option<Box<dyn Any>>)>)>
}

impl<T, TState, TComponentManager> EventTracker<T, TState, TComponentManager>
{
    pub fn new() -> EventTracker<T, TState, TComponentManager>
    {
        return EventTracker {
            events: Vec::new()
        };
    }

    pub fn push<TRes: 'static, TFunc: 'static + Fn(&mut T, &mut CommonContext<TState, TComponentManager>, Option<TRes>)>(&mut self, handle: Handle, func: TFunc)
    {
        self.events.push((handle, Box::new(move |this, ctx, data| {
            if let Some(obj) = data {
                let o = *obj.downcast().unwrap();
                func(this, ctx, o);
            } else {
                func(this, ctx, None);
            }
        })));
    }

    pub fn poll_batch(&mut self, event_manager: &mut EventManager<TState, TComponentManager>) -> EventTrackerBatch<T, TState, TComponentManager>
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
        return EventTrackerBatch {
            events: batch
        };
    }
}

pub struct EventTrackerBatch<T, TState, TComponentManager>
{
    events: Vec<(Option<Box<dyn Any>>, Box<dyn Fn(&mut T, &mut CommonContext<TState, TComponentManager>, Option<Box<dyn Any>>)>)>
}

impl<T, TState, TComponentManager> EventTrackerBatch<T, TState, TComponentManager>
{
    pub fn run(self, this: &mut T, ctx: &mut CommonContext<TState, TComponentManager>)
    {
        for (data, func) in self.events {
            func(this, ctx, data);
        }
    }
}

pub struct Event
{
    pub sender: Option<ObjectRef>,
    pub target: Option<ObjectRef>,
    pub data: Box<dyn Any>,
    pub tracking: bool,
    pub handle: Handle,
}

pub struct EventBuilder
{
    ev: Event,
}

impl EventBuilder
{
    pub fn new<TEvent: Any>(event: TEvent) -> EventBuilder
    {
        return EventBuilder
        {
            ev: Event {
                sender: None,
                target: None,
                data: Box::from(event),
                tracking: false,
                handle: 0,
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

pub enum SystemEvent<TState, TComponentManager>
{
    EnableUpdate(ObjectRef, bool),
    Serialize(ObjectRef),
    Deserialize(ObjectRef, bpx::sd::Object),
    Spawn(Box<dyn CoreObject<TState, TComponentManager>>),
    Destroy(ObjectRef),
}

pub struct EventManager<TState, TComponentManager>
{
    events: VecDeque<Event>,
    system_events: VecDeque<(bool, Handle, SystemEvent<TState, TComponentManager>)>,
    cur_handle: Handle,
    event_responses: HashMap<Handle, Option<Box<dyn Any>>>,
}

impl<TState, TComponentManager> EventManager<TState, TComponentManager>
{
    pub fn new() -> EventManager<TState, TComponentManager>
    {
        return EventManager {
            events: VecDeque::new(),
            system_events: VecDeque::new(),
            cur_handle: 0,
            event_responses: HashMap::new(),
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

    pub fn system(&mut self, event: SystemEvent<TState, TComponentManager>, tracking: bool) -> Handle
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

    pub fn poll_system_event(&mut self) -> Option<(bool, Handle, SystemEvent<TState, TComponentManager>)>
    {
        return self.system_events.pop_front();
    }

    pub fn queue_response(&mut self, handle: Handle, response: Option<Box<dyn Any>>)
    {
        self.event_responses.insert(handle, response);
    }
}

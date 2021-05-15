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

use std::any::Any;
use std::boxed::Box;

use crate::object::ObjectRef;

pub struct EventContext<'a, TState, TComponentManager>
{
    pub ptr: ObjectRef,
    pub other: Option<ObjectRef>,
    pub state: &'a mut TState,
    pub components: &'a mut TComponentManager
}

pub struct EventResult
{
    to_send: Vec<(Option<ObjectRef>, Box<dyn Any>)>,
    remove_flag: bool
}

impl EventResult
{
    pub fn new() -> EventResult
    {
        return EventResult
        {
            to_send: Vec::new(),
            remove_flag: false
        };
    }

    pub fn remove(&mut self)
    {
        self.remove_flag = true;
    }

    pub fn send<EventType: Any>(&mut self, target: ObjectRef, ev: EventType)
    {
        self.to_send.push((Some(target), Box::from(ev)));
    }

    pub fn broadcast<EventType: Any>(&mut self, ev: EventType)
    {
        self.to_send.push((None, Box::from(ev)));
    }
}

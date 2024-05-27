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

use crate::event::{Builder, EventManager};
use crate::object::ObjectRef;
use crate::scene::Configuration;

pub enum Type<B> {
    EnableObject(bool),
    RemoveObject,
    SpawnObject(B),
}

pub enum Notify {
    Sender(ObjectRef),
    All,
    None,
}

impl Notify {
    pub fn into_builder<C: Configuration>(self, ty: Type<C::Builder>) -> Builder<Event<C>> {
        match self {
            Notify::Sender(v) => Builder::new(Event { notify: true, ty }).sender(v),
            Notify::All => Builder::new(Event { notify: true, ty }),
            Notify::None => Builder::new(Event { notify: false, ty }),
        }
    }
}

pub struct Event<C: Configuration> {
    pub notify: bool,
    pub ty: Type<C::Builder>,
}

impl<C: Configuration> EventManager<Event<C>> {
    pub fn enable_object(&mut self, notify: Notify, target: ObjectRef, enable: bool) {
        let builder = notify
            .into_builder(Type::EnableObject(enable))
            .target(target);
        self.send(builder);
    }

    pub fn remove_object(&mut self, notify: Notify, target: ObjectRef) {
        let builder = notify
            .into_builder(super::event::Type::RemoveObject)
            .target(target);
        self.send(builder);
    }

    pub fn spawn_object(&mut self, notify: Notify, builder: C::Builder) {
        let builder = notify.into_builder(super::event::Type::SpawnObject(builder));
        self.send(builder);
    }
}

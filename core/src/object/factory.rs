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

use std::marker::PhantomData;
use crate::event::Event;
use crate::object::{Context, New, Object, ObjectRef};
use crate::scene::{Interface, ObjectContext};

pub trait Factory<C: Context> {
    type Object: Object<C>;

    fn spawn(self, ctx: &mut C, state: &C::AppState, this: ObjectRef) -> Self::Object;
    fn can_update_object(&self) -> bool;
}

pub struct NullObject;

impl<C: Context> Object<C> for NullObject {
    fn on_event(&mut self, _: &mut C, _: &C::AppState, _: &Event<C::Event>) {
    }

    fn on_remove(&mut self, _: &mut C, _: &C::AppState) {
    }

    fn on_update(&mut self, _: &mut C, _: &C::AppState) {
    }

    fn class(&self) -> &str {
        "null"
    }
}

impl<C: Context> New<C> for NullObject {
    type Arguments = ();

    fn new(_: &mut C, _: &C::AppState, _: ObjectRef, _: Self::Arguments) -> Self {
        Self
    }
}

pub struct NullFactory<I: Interface> {
    useless: PhantomData<I>
}

impl<I: Interface> Factory<ObjectContext<I>> for NullFactory<I> {
    type Object = NullObject;

    fn spawn(self, _: &mut ObjectContext<I>, _: &I::AppState, _: ObjectRef) -> Self::Object {
        NullObject
    }

    fn can_update_object(&self) -> bool {
        false
    }
}

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

use crate::object::{Context, Object, ObjectRef};

pub type Function<C> = Box<dyn FnOnce(&mut C, &<C as crate::system::Context>::AppState, ObjectRef) -> Box<dyn Object<C>>>;

pub struct Factory<C: Context> {
    func: Function<C>,
    updates: bool
}

impl<C: Context> Factory<C> {
    pub fn updates(&self) -> bool {
        self.updates
    }

    pub fn invoke(self, ctx: &mut C, state: &C::AppState, this_ref: ObjectRef) -> Box<dyn Object<C>> {
        (self.func)(ctx, state, this_ref)
    }

    pub fn new_static<O: 'static + Object<C>, F: 'static + FnOnce(&mut C, &C::AppState, ObjectRef) -> O>(func: F) -> Self {
        Self {
            func: Box::new(|ctx, state, this_ref| Box::new(func(ctx, state, this_ref))),
            updates: false
        }
    }

    pub fn new_dynamic(func: Function<C>) -> Self {
        Self {
            func,
            updates: false
        }
    }

    pub fn set_updates(mut self, flag: bool) -> Self {
        self.updates = flag;
        self
    }
}

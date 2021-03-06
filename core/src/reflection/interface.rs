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

use crate::object::{Context, CoreObject, ObjectRef};

pub trait PropertyGroup
{
    fn deserialize(obj: bpx::sd::Object) -> Self;
}

pub trait PropertyInitializer
{
    fn initialize(&mut self, obj: bpx::sd::Object);
}

//impl <TContext: Context, TObject: 'static + CoreObject<TContext>>

/*pub trait ClassBuilder<TContext: Context, TObject: 'static + CoreObject<TContext>>
{
    fn name() -> String;
    fn init_func() -> Box<dyn Fn (Option<bpx::sd::Object>) -> Box<dyn CoreObject<TContext>>>;
}

pub struct Class<TContext: Context>
{
    name: String,
    func: Box<dyn Fn (Option<bpx::sd::Object>) -> Box<dyn CoreObject<TContext>>>
}

impl<TContext: Context, TObject: 'static + CoreObject<TContext> + ClassConnector> Class<TContext>
{

}*/

pub trait ClassInitializer
{
    fn new_instance(this: ObjectRef) -> Self;
}

impl<T: From<ObjectRef>> ClassInitializer for T
{
    fn new_instance(this: ObjectRef) -> Self
    {
        return Self::from(this);
    }
}

pub trait ClassConnector : ClassInitializer
{
    fn class_name() -> &'static str;
}

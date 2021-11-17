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

//! The ObjectList system provides a simple way to instantiate objects by their class names

use crate::object::{CoreObject, Context, ObjectRef, ObjectFactory};
use std::collections::HashMap;
use crate::system::System;
use crate::reflection::interface::{ClassConnector, PropertyInitializer};

pub struct ObjectRegistry<TContext: Context>
{
    classes: HashMap<String, Box<dyn Fn (Option<bpx::sd::Object>) -> ObjectFactory<TContext>>>
}

impl<TContext: Context> System<TContext::SystemContext> for ObjectRegistry<TContext>
{
    fn update(&mut self, _: &TContext::SystemContext, _: &<TContext::SystemContext as crate::system::Context>::AppState)
    {
    }
}

impl<TContext: Context> Default for ObjectRegistry<TContext>
{
    fn default() -> Self
    {
        return ObjectRegistry {
            classes: HashMap::new()
        };
    }
}

impl<TContext: Context> ObjectRegistry<TContext>
{
    pub fn create(&self, class: &str, props: Option<bpx::sd::Object>) -> Option<ObjectFactory<TContext>>
    {
        if let Some(func) = self.classes.get(class)
        {
            return Some(func(props));
        }
        return None;
    }

    pub fn register<TObject: 'static + CoreObject<TContext> + ClassConnector>(&mut self)
    {
        self.classes.insert(TObject::class_name().to_string(), Box::new(|_| {
            return ObjectFactory::from(|this| TObject::new_instance(this));
        }));
    }

    pub fn register_with_props<TObject: 'static + CoreObject<TContext> + ClassConnector + PropertyInitializer>(&mut self)
    {
        self.classes.insert(TObject::class_name().to_string(), Box::new(|props| {
            return ObjectFactory::from(|this| {
                let mut obj = TObject::new_instance(this);
                if let Some(props) = props {
                    obj.initialize(props);
                }
                return obj;
            });
        }));
    }
}

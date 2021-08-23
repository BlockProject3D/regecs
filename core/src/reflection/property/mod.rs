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

use crate::reflection::class::Class;

pub mod config;

#[derive(Clone)]
pub struct PropertyType
{
    pub type_name: &'static str,
    pub class: Option<Class>
}

pub trait AsProperty
{
    type ConfigType : 'static + config::PropertyConfig;
    fn prop_type() -> PropertyType;
}

pub struct Property
{
    name: String,
    optional: bool,
    ptype: PropertyType,
    config: Box<dyn config::PropertyConfig>
}

impl Clone for Property
{
    fn clone(&self) -> Self
    {
        return Property
        {
            name: self.name.clone(),
            optional: self.optional,
            ptype: self.ptype.clone(),
            config: self.config.clone_box()
        }
    }
}

impl Property
{
    fn new<T: AsProperty>(name: String, optional: bool, config: T::ConfigType) -> Property
    {
        return Property
        {
            name,
            optional,
            ptype: T::prop_type(),
            config: Box::new(config)
        }
    }
}

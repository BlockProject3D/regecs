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

pub struct Property<T: Default + AsProperty>
{
    value: T,
    changed: bool,
    config: T::PropertyConfig
}

impl<T: Default + AsProperty> Property<T>
{
    pub fn new(config: T::PropertyConfig) -> Property<T, T::PropertyConfig>
    {
        return Property {
            value: T::default(),
            changed: false,
            config
        };
    }

    pub fn get(&self) -> &T
    {
        return &self.value;
    }

    pub fn config(&self) -> &T::PropertyConfig
    {
        return &self.config;
    }

    pub fn set(&mut self, new_value: T)
    {
        self.value = new_value;
        self.changed = true;
    }

    pub fn poll(&mut self) -> Option<&T>
    {
        if !self.changed {
            return None;
        } else {
            self.changed = false;
            return Some(&self.value);
        }
    }
}

pub trait Serializable
{
    fn serialize(&self) -> bpx::sd::Value;
}

pub trait Deserializable
{
    fn deserialize(&mut self, val: &bpx::sd::Value);
}

pub trait AsProperty : Serializable + Deserializable
{
    type PropConfig : Serializable;
    const PROP_TYPE_NAME: &'static str;
}

pub struct NumericProperty<T>
{
    pub min: T,
    pub max: T,
    pub step: T
}

pub struct StringProperty
{
    max_chars: u32
}

macro_rules! impl_numeric_prop_provider {
    ($t: ty) => {
        impl AsProperty for $t
        {
            type PropConfig = NumericProperty<$t>;

            const PROP_TYPE_NAME: &'static str = stringify!($t);
        }
    };
}

impl_numeric_prop_provider!(i8);
impl_numeric_prop_provider!(i16);
impl_numeric_prop_provider!(i32);
impl_numeric_prop_provider!(i64);
impl_numeric_prop_provider!(u8);
impl_numeric_prop_provider!(u16);
impl_numeric_prop_provider!(u32);
impl_numeric_prop_provider!(u64);
impl_numeric_prop_provider!(f32);
impl_numeric_prop_provider!(f64);

impl AsProperty for bool
{
    type PropConfig = ();

    const PROP_TYPE_NAME: &'static str = "bool";
}

impl AsProperty for String
{
    type PropConfig = StringProperty;

    const PROP_TYPE_NAME: &'static str = "String";
}

pub trait PropertyGroup : Serializable + Deserializable
{
    /// List properties
    ///
    /// # Returns
    ///
    /// * an array of all properties as (name, type, config) tuples
    fn list_properties(&self) -> Vec<(&'static str, String, bpx::sd::Value)>;
}

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

//! REGECS reflection system.
//!
//! This module is currently unfinished. This is intended to support instantiating objects from a
//! level editor.

//pub mod class;
//pub mod interface;
//pub mod property;

pub mod property {
    use std::borrow::Cow;

    //TODO: Add Signed and Unsigned enums with support for writing and reading signed and unsigned
    // numbers with number of bits (support for network compression).

    pub trait Reader {
        type Error;

        fn read_null(&mut self) -> Result<bool, Self::Error>;
        fn read_u8(&mut self) -> Result<u8, Self::Error>;
        fn read_u16(&mut self) -> Result<u16, Self::Error>;
        fn read_u32(&mut self) -> Result<u32, Self::Error>;
        fn read_u64(&mut self) -> Result<u64, Self::Error>;
        fn read_i8(&mut self) -> Result<i8, Self::Error>;
        fn read_i16(&mut self) -> Result<i16, Self::Error>;
        fn read_i32(&mut self) -> Result<i32, Self::Error>;
        fn read_i64(&mut self) -> Result<i64, Self::Error>;
        fn read_f32(&mut self) -> Result<f32, Self::Error>;
        fn read_f64(&mut self) -> Result<f64, Self::Error>;
        fn read_bool(&mut self) -> Result<bool, Self::Error>;
        fn read_string(&mut self) -> Result<Cow<str>, Self::Error>;
    }

    pub trait Writer {
        type Error;

        fn write_null(&mut self) -> Result<(), Self::Error>;
        fn write_u8(&mut self, val: u8) -> Result<(), Self::Error>;
        fn write_u16(&mut self, val: u16) -> Result<(), Self::Error>;
        fn write_u32(&mut self, val: u32) -> Result<(), Self::Error>;
        fn write_u64(&mut self, val: u64) -> Result<(), Self::Error>;
        fn write_i8(&mut self, val: i8) -> Result<(), Self::Error>;
        fn write_i16(&mut self, val: i16) -> Result<(), Self::Error>;
        fn write_i32(&mut self, val: i32) -> Result<(), Self::Error>;
        fn write_i64(&mut self, val: i64) -> Result<(), Self::Error>;
        fn write_f32(&mut self, val: f32) -> Result<(), Self::Error>;
        fn write_f64(&mut self, val: f64) -> Result<(), Self::Error>;
        fn write_bool(&mut self, val: bool) -> Result<(), Self::Error>;
        fn write_string(&mut self, val: &str) -> Result<(), Self::Error>;
    }

    pub trait Object: Sized + Send {
        // This must be Send otherwise it's impossible to safely write the
        // object properties as a network packet.
        fn read<T: Reader>(reader: &mut T) -> Result<Self, T::Error>;
        fn write<T: Writer>(&self, writer: &mut T) -> Result<(), T::Error>;
    }
}

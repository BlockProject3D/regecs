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

//! REGECS system layer

mod interface;
//mod object_registry;

pub use interface::Context;
pub use interface::System;
pub use interface::Updatable;
pub use interface::SystemList;
pub use interface::SystemProvider;
//pub use object_registry::ObjectRegistry;

pub struct SystemType<TSystem: System>
{
    useless: std::marker::PhantomData<TSystem>
}

impl<TSystem: System> SystemType<TSystem>
{
    pub fn new() -> SystemType<TSystem>
    {
        return SystemType {
            useless: std::marker::PhantomData::default()
        };
    }
}

pub trait SystemTypeProvider<TSystem: System>
{
    fn class() -> SystemType<TSystem>;
}

impl<TSystem: System> SystemTypeProvider<TSystem> for TSystem
{
    fn class() -> SystemType<TSystem>
    {
        return SystemType::<TSystem>::new();
    }
}

pub trait SystemPart<TSystem: System>
{
    fn get(&self, _: SystemType<TSystem>) -> &TSystem;
    fn get_mut(&mut self, _: SystemType<TSystem>) -> &mut TSystem;
}

impl<TSystem: System, TSystemList: SystemProvider<TSystem>> SystemPart<TSystem> for TSystemList
{
    fn get(&self, _: SystemType<TSystem>) -> &TSystem
    {
        return self.system();
    }

    fn get_mut(&mut self, _: SystemType<TSystem>) -> &mut TSystem
    {
        return self.system_mut();
    }
}

pub fn get_system<TSystemList: SystemProvider<TSystem>, TSystem>(systems: &TSystemList) -> &TSystem
{
    return systems.system();
}

pub fn get_system_mut<TSystemList: SystemProvider<TSystem>, TSystem>(systems: &mut TSystemList) -> &mut TSystem
{
    return systems.system_mut();
}

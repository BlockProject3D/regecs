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

use crate::event::EventManager;
use crate::component::ComponentManager;
use crate::object::ObjectTree;

pub trait Context
{
    type AppState;
    type ComponentManager: ComponentManager;
    type Context: crate::object::Context;

    fn components(&self) -> &Self::ComponentManager;
    fn components_mut(&mut self) -> &mut Self::ComponentManager;
    fn event_manager(&mut self) -> &mut EventManager<Self::Context>;
    fn objects(&self) -> &ObjectTree;
}

/// System interface
pub trait System {}

/// System interface
pub trait Updatable<TContext: Context>
{
    fn update(&mut self, ctx: &mut TContext, state: &TContext::AppState);
}

/// System manager interface
pub trait SystemManager<TContext: Context>
{
    fn update(&mut self, ctx: &mut TContext, state: &TContext::AppState);
}

pub trait SystemProvider<TSystem>
{
    fn system(&self) -> &TSystem;
    fn system_mut(&mut self) -> &mut TSystem;
}

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

impl<TSystem: System, TSystemManager: SystemProvider<TSystem>> SystemPart<TSystem> for TSystemManager
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

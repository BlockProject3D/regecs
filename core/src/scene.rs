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

//! REGECS scene object

use crate::object::ObjectRef;
use crate::system::System;
use crate::object::LowObject;

/// Represents a scene, provides storage for systems and objects
pub struct Scene<TState, TComponentManager>
{
    component_manager: TComponentManager,
    systems: Vec<Box<dyn System<TState, TComponentManager>>>,
    objects: Vec<Box<dyn LowObject<TState, TComponentManager>>>
}

impl <TState, TComponentManager> Scene<TState, TComponentManager>
{
    pub fn new(component_manager: TComponentManager) -> Scene<TState, TComponentManager>
    {
        return Scene
        {
            component_manager: component_manager,
            systems: Vec::new(),
            objects: Vec::new()
        };
    }

    pub fn update(&mut self, ctx: &mut TState)
    {
        for i in 0..self.systems.len()
        {
            self.systems[i].update(ctx, &mut self.component_manager);
        }
    }

    pub fn add_system<TSystem: 'static + System<TState, TComponentManager>>(&mut self, system: TSystem)
    {
        let b = Box::new(system);
        self.systems.push(b);
    }

    pub fn consume(self) -> TComponentManager
    {
        return self.component_manager;
    }
}

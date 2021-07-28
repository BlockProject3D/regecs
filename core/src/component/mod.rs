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

//! REGECS component layer

pub mod interface;

mod basic_pool;
mod grouped_pool;

pub use basic_pool::BasicComponentPool;
pub use grouped_pool::GroupComponentPool;

use crate::component::interface::{Component, ComponentPool, ComponentProvider};

#[macro_export]
macro_rules! pool_type {
    ($i: ty) => {
        <$i as regecs::component::interface::Component>::Pool
    };
}

pub fn add_component<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &mut TComponentManager,
    comp: TComponent
) -> usize
{
    return mgr.pool_mut().add(comp);
}

pub fn get_component<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &TComponentManager,
    id: usize
) -> &TComponent
{
    return &mgr.pool()[id];
}

pub fn get_component_mut<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &mut TComponentManager,
    id: usize
) -> &mut TComponent
{
    return &mut mgr.pool_mut()[id];
}

pub fn remove_component<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &mut TComponentManager,
    id: usize
)
{
    mgr.pool_mut().remove(id);
}

pub fn get_component_pool_mut<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &mut TComponentManager
) -> &mut TComponent::Pool
{
    return mgr.pool_mut();
}

pub fn get_component_pool<TComponentManager: ComponentProvider<TComponent>, TComponent: Component>(
    mgr: &TComponentManager
) -> &TComponent::Pool
{
    return mgr.pool();
}

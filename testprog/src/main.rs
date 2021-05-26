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

use regecs::{
    component::{add_component, get_component, remove_component, ComponentPool, ComponentProvider},
    scene::Scene,
    system::{EventList, System}
};
use regecs_codegen::ComponentManager;

mod components
{
    pub struct Test
    {
        pub value: i32
    }

    pub struct Test2
    {
        pub value2: i32
    }
}

struct MySystem {}

impl<TComponentManager: ComponentProvider<components::Test> + ComponentProvider<components::Test2>>
    System<i32, TComponentManager> for MySystem
{
    fn update(&mut self, ctx: &mut i32, components: &mut TComponentManager) -> Option<EventList>
    {
        get_component::<_, components::Test>(components, 0).value = 12;
        get_component::<_, components::Test2>(components, 0).value2 = 42;
        *ctx = 1;
        return None;
    }
}

#[derive(ComponentManager)]
struct MyComponentManager
{
    pool: ComponentPool<components::Test>,
    pool1: ComponentPool<components::Test2>
}

fn main()
{
    let mut ctx = 0;
    let mut mgr = MyComponentManager::new();
    add_component(&mut mgr, components::Test { value: 0 });
    add_component(&mut mgr, components::Test2 { value2: 0 });
    let mut sc: Scene<i32, _> = Scene::new(mgr);
    sc.add_system(MySystem {});
    sc.update(&mut ctx);
    assert_eq!(ctx, 1);
    mgr = sc.consume();
    assert_eq!(get_component::<_, components::Test>(&mut mgr, 0).value, 12);
    assert_eq!(get_component::<_, components::Test2>(&mut mgr, 0).value2, 42);
    remove_component::<_, components::Test>(&mut mgr, 0);
    remove_component::<_, components::Test2>(&mut mgr, 0);
    let test = <MyComponentManager as ComponentProvider<components::Test>>::get_pool(&mut mgr).size();
    let test1 = <MyComponentManager as ComponentProvider<components::Test2>>::get_pool(&mut mgr).size();
    assert_eq!(test, 0);
    assert_eq!(test1, 0);
}

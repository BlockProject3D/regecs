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
    component::{
        add_component,
        get_component,
        get_component_mut,
        interface::{ComponentPool, ComponentProvider},
        remove_component
    },
    pool_type,
    scene::Scene,
    system::{EventList, System}
};
use regecs_codegen::ComponentManager;

use crate::components::ComplexComponent;

mod components
{
    use regecs::{
        component::{
            interface::{Component, ComponentProvider, IterableComponentPool},
            BasicComponentPool,
            GroupComponentPool
        },
        system::{EventList, System}
    };

    pub struct Test
    {
        pub value: i32
    }

    impl Component for Test
    {
        type Pool = BasicComponentPool<Test>;
    }

    pub struct Test2
    {
        pub value2: i32
    }

    impl Component for Test2
    {
        type Pool = BasicComponentPool<Test2>;
    }

    pub struct ComplexComponent
    {
        last_order: u32,
        pub order: u32,
        pub value: i32
    }

    impl ComplexComponent
    {
        pub fn new(order: u32, value: i32) -> ComplexComponent
        {
            return ComplexComponent {
                last_order: 0,
                order,
                value
            };
        }
    }

    impl Component for ComplexComponent
    {
        type Pool = GroupComponentPool<u32, ComplexComponent>;
    }

    pub struct ComplexSystem
    {
        events: Vec<(usize, u32)>
    }

    impl ComplexSystem
    {
        pub fn new() -> ComplexSystem
        {
            return ComplexSystem {
                events: Vec::with_capacity(5)
            };
        }
    }

    impl<TComponentManager: ComponentProvider<ComplexComponent>> System<i32, TComponentManager> for ComplexSystem
    {
        fn update(&mut self, _: &mut i32, components: &mut TComponentManager) -> Option<EventList>
        {
            println!("____");
            while let Some((component, new_order)) = self.events.pop() {
                components.pool_mut().update_group(component, new_order);
            }
            for (i, v) in components.pool_mut().iter_mut() {
                if v.last_order != v.order {
                    // Record new events
                    self.events.push((i, v.order));
                    v.last_order = v.order;
                }
                println!("{}", i);
            }
            return None;
        }
    }
}

struct MySystem {}

impl<TComponentManager: ComponentProvider<components::Test> + ComponentProvider<components::Test2>>
    System<i32, TComponentManager> for MySystem
{
    fn update(&mut self, ctx: &mut i32, components: &mut TComponentManager) -> Option<EventList>
    {
        get_component_mut::<_, components::Test>(components, 0).value = 12;
        get_component_mut::<_, components::Test2>(components, 0).value2 = 42;
        *ctx = 1;
        return None;
    }
}

#[derive(ComponentManager)]
struct MyComponentManager
{
    pool: pool_type!(components::Test),
    pool1: pool_type!(components::Test2),
    pool2: pool_type!(components::ComplexComponent)
}

fn main()
{
    let mut ctx = 0;
    let mut mgr = MyComponentManager::new();
    add_component(&mut mgr, components::Test { value: 0 });
    add_component(&mut mgr, components::Test2 { value2: 0 });
    add_component(&mut mgr, ComplexComponent::new(2, 3));
    add_component(&mut mgr, ComplexComponent::new(1, 1));
    add_component(&mut mgr, ComplexComponent::new(2, 4));
    add_component(&mut mgr, ComplexComponent::new(1, 2));
    let mut sc: Scene<i32, _> = Scene::new(mgr);
    sc.add_system(MySystem {});
    sc.update(&mut ctx);
    assert_eq!(ctx, 1);
    sc.add_system(components::ComplexSystem::new());
    ctx = 0;
    sc.update(&mut ctx);
    sc.update(&mut ctx);
    mgr = sc.consume();
    assert_eq!(get_component::<_, components::Test>(&mut mgr, 0).value, 12);
    assert_eq!(get_component::<_, components::Test2>(&mut mgr, 0).value2, 42);
    remove_component::<_, components::Test>(&mut mgr, 0);
    remove_component::<_, components::Test2>(&mut mgr, 0);
    let test = <MyComponentManager as ComponentProvider<components::Test>>::pool(&mut mgr).size();
    let test1 = <MyComponentManager as ComponentProvider<components::Test2>>::pool(&mut mgr).size();
    assert_eq!(test, 0);
    assert_eq!(test1, 0);
}

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

use components::ComplexSystem;
use regecs::{
    entity::{Entity, EntityPart},
    macros::build_system_manager,
    scene::Scene,
    system::{System, SystemPart, Updatable}
};
use regecs::component::ComponentRef;
use regecs::component::pool::ComponentManager;
use regecs::component::pool::ComponentPool;

use crate::components::{ComplexComponent, TestComponentManager};

mod components
{
    use regecs::{
        component::{
            pool::{BasicComponentPool, GroupComponentPool},
            Component
        },
        macros::build_component_manager,
        system::{System, Updatable}
    };
    use regecs::component::ComponentRef;
    use regecs::component::pool::{ComponentManager, Iter};

    pub struct Test
    {
        pub value: i32
    }

    impl Component for Test
    {
        type Pool = BasicComponentPool<Test>;
    }

    //impl ClassConnector for Test
    //{
    //    fn class() -> Class
    //    {
    /*static CLASS_STORAGE: &'static Class = &Class::new(String::from("Test"), Vec::new(), Test::new_instance);
    return &CLASS_STORAGE;*/
    //        return Class::new(String::from("Test"), Vec::new(), Test::new_instance);
    //    }

    //    fn new_instance() -> Box<dyn Any>
    //    {
    //        todo!()
    //    }
    //}

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

    build_component_manager!(
        #[derive(Default)]
        pub TestComponentManager {
            (attachments) tests: Test,
            (attachments) test2s: Test2,
            (attachments) complexes: ComplexComponent
        }
        into (Test, Test2) => (tests, test2s)
    );

    pub struct ComplexSystem
    {
        events: Vec<(ComponentRef<ComplexComponent>, u32)>
    }

    impl Default for ComplexSystem
    {
        fn default() -> ComplexSystem
        {
            return ComplexSystem {
                events: Vec::with_capacity(5)
            };
        }
    }

    impl System for ComplexSystem {}

    impl<TContext: regecs::system::Context> Updatable<TContext> for ComplexSystem
    where
        TContext::ComponentManager: ComponentManager<ComplexComponent>
    {
        fn update(&mut self, ctx: &mut TContext, _: &TContext::AppState)
        {
            println!("____");
            while let Some((component, new_order)) = self.events.pop() {
                ctx.components_mut().get_mut().update_group(component, new_order);
            }
            for (i, v) in ctx.components_mut().get_mut().iter_mut() {
                if v.last_order != v.order {
                    // Record new events
                    self.events.push((i, v.order));
                    v.last_order = v.order;
                }
                println!("{}, {}", i, v.value);
            }
        }
    }
}

struct MySystem
{
    pub val: i32
}

impl Default for MySystem
{
    fn default() -> MySystem
    {
        return MySystem { val: 0 };
    }
}

impl System for MySystem {}

impl<TContext: regecs::system::Context<AppState = i32>> Updatable<TContext> for MySystem
where
    TContext::ComponentManager:
        ComponentManager<components::Test> + ComponentManager<components::Test2>
{
    fn update(&mut self, ctx: &mut TContext, state: &TContext::AppState)
    {
        let test: ComponentRef<components::Test> = ComponentRef::new(0);
        let test2: ComponentRef<components::Test2> = ComponentRef::new(0);
        ctx.components_mut().get_component_mut(test).value = 12;
        ctx.components_mut().get_component_mut(test2).value2 = 42;
        assert_eq!(
            ctx.components().get_component(test2).value2,
            42
        );
        assert_eq!(*state, 42);
    }
}

#[derive(Default)]
struct MySystem2 {}

impl System for MySystem2 {}

build_system_manager!(
    #[derive(Default)]
    pub TestSystemManager<i32, components::TestComponentManager>
    {
        (updates) my: MySystem,
        (updates) complex: ComplexSystem,
        my2: MySystem2
    }
    into (MySystem, ComplexSystem) => (my, complex)
    into (MySystem) => (my)
    into (ComplexSystem) => (complex)
    context MyContext
);

fn main()
{
    let ctx = 42;
    let mut mgr = components::TestComponentManager::default();
    let mut entity = Entity::new(&mut mgr, 0);
    let test = entity.add(components::Test { value: 12 });
    entity.get_mut(test).value = 1;
    let test1 = mgr.add_component(components::Test { value: 0 });
    let test2 = mgr.add_component(components::Test2 { value2: 0 });
    mgr.add_component(ComplexComponent::new(2, 3));
    mgr.add_component(ComplexComponent::new(1, 1));
    mgr.add_component(ComplexComponent::new(2, 4));
    mgr.add_component(ComplexComponent::new(1, 2));
    let mut systems = TestSystemManager::default();
    {
        use regecs::system::SystemTypeProvider;
        systems.get_mut(MySystem::class()).val = 42;
    }
    let mut sc: Scene<i32, _, _> = Scene::new(mgr, systems);
    sc.update(&ctx);
    sc.update(&ctx);
    mgr = sc.consume();
    assert_eq!(mgr.get_component(test).value, 12);
    assert_eq!(
        mgr.get_component(test2).value2,
        42
    );
    mgr.remove_component(test);
    mgr.remove_component(test2);
    let sfdk = <TestComponentManager as ComponentManager<components::Test>>::get(&mgr).len();
    let fh = <TestComponentManager as ComponentManager<components::Test2>>::get(&mgr).len();
    assert_eq!(sfdk, 1);
    assert_eq!(fh, 0);
    mgr.remove_component(test1);
    let test = <TestComponentManager as ComponentManager<components::Test>>::get(&mgr).len();
    assert_eq!(test, 0);
}

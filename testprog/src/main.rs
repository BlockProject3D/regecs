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
use regecs::component::pool::ComponentManager;
use regecs::component::pool::ComponentPool;
use regecs::component::ComponentRef;
use regecs::scene::SystemContext;
use regecs::system::Update;
use regecs::{
    entity::{Entity, EntityPart},
    scene::Scene,
};

use crate::components::ComplexComponent;

mod components {
    use regecs::component::pool::{Attachments, ComponentManager, Iter};
    use regecs::component::{
        pool::{BasicComponentPool, GroupComponentPool},
        Component,
    };
    use regecs::component::{Clear, ComponentRef, Pool};
    use regecs::object::ObjectRef;
    use regecs::system::Update;

    pub struct Test {
        pub value: i32,
    }

    impl Component for Test {
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

    pub struct Test2 {
        pub value2: i32,
    }

    impl Component for Test2 {
        type Pool = BasicComponentPool<Test2>;
    }

    pub struct ComplexComponent {
        last_order: u32,
        pub order: u32,
        pub value: i32,
    }

    impl ComplexComponent {
        pub fn new(order: u32, value: i32) -> ComplexComponent {
            return ComplexComponent {
                last_order: 0,
                order,
                value,
            };
        }
    }

    impl Component for ComplexComponent {
        type Pool = GroupComponentPool<u32, ComplexComponent>;
    }

    #[derive(Default)]
    pub struct TestComponentManager {
        tests: Pool<Test>,
        test2s: Pool<Test2>,
        complexes: Pool<ComplexComponent>,
    }

    regecs::impl_component_manager!(TestComponentManager { (tests: Test) (test2s: Test2) (complexes: ComplexComponent) });

    // TODO: Implement a derive proc macro for Clear
    impl Clear for TestComponentManager {
        fn clear(&mut self, entity: ObjectRef) {
            self.tests.clear(entity);
            self.test2s.clear(entity);
            self.complexes.clear(entity);
        }
    }

    pub struct ComplexSystem {
        events: Vec<(ComponentRef<ComplexComponent>, u32)>,
    }

    impl Default for ComplexSystem {
        fn default() -> ComplexSystem {
            return ComplexSystem {
                events: Vec::with_capacity(5),
            };
        }
    }

    impl<C: regecs::system::Context> Update<C> for ComplexSystem
    where
        C::ComponentManager: ComponentManager<ComplexComponent>,
    {
        fn update(&mut self, ctx: &mut C, _: &C::AppState) {
            println!("____");
            while let Some((component, new_order)) = self.events.pop() {
                ctx.components_mut()
                    .get_mut()
                    .update_group(component, new_order);
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

struct MySystem {
    pub val: i32,
}

impl Default for MySystem {
    fn default() -> MySystem {
        return MySystem { val: 0 };
    }
}

impl<C: regecs::system::Context<AppState = i32>> Update<C> for MySystem
where
    C::ComponentManager: ComponentManager<components::Test> + ComponentManager<components::Test2>,
{
    fn update(&mut self, ctx: &mut C, state: &C::AppState) {
        let test: ComponentRef<components::Test> = ComponentRef::new(0);
        let test2: ComponentRef<components::Test2> = ComponentRef::new(0);
        ctx.components_mut().get_component_mut(test).value = 12;
        ctx.components_mut().get_component_mut(test2).value2 = 42;
        assert_eq!(ctx.components().get_component(test2).value2, 42);
        assert_eq!(*state, 42);
    }
}

#[derive(Default)]
struct MySystem2 {}

#[derive(Default)]
pub struct TestSystemManager {
    my: MySystem,
    complex: ComplexSystem,
    my2: MySystem2,
}

type Ctx = SystemContext<TestSystemManager, components::TestComponentManager, (), i32>;

impl Update<Ctx> for TestSystemManager {
    fn update(&mut self, ctx: &mut Ctx, state: &i32) {
        self.my.update(ctx, state);
        self.complex.update(ctx, state);
    }
}

fn main() {
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
    systems.my.val = 42;
    let mut sc: Scene<_, _, (), i32> = Scene::new(mgr, systems);
    sc.update(&ctx);
    sc.update(&ctx);
    mgr = sc.consume();
    assert_eq!(mgr.get_component(test).value, 12);
    assert_eq!(mgr.get_component(test2).value2, 42);
    mgr.remove_component(test);
    mgr.remove_component(test2);
    let sfdk =
        <components::TestComponentManager as ComponentManager<components::Test>>::get(&mgr).len();
    let fh =
        <components::TestComponentManager as ComponentManager<components::Test2>>::get(&mgr).len();
    assert_eq!(sfdk, 1);
    assert_eq!(fh, 0);
    mgr.remove_component(test1);
    let test =
        <components::TestComponentManager as ComponentManager<components::Test>>::get(&mgr).len();
    assert_eq!(test, 0);
}

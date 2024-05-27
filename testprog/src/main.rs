// Copyright (c) 2024, BlockProject 3D
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
use regecs::component::{ComponentPool, ComponentRef};
use regecs::entity::EntityIndex;
use regecs::event::Event;
use regecs::object::{Class, Object, ObjectRef};
use regecs::scene::Scene;
use regecs::scene::state;
use regecs::system::Update;

use crate::components::ComplexComponent;

mod components {
    use regecs::component::list::Iter;
    use regecs::component::{list::{BasicComponentList, GroupComponentList}, Component, ComponentPool, Clear};
    use regecs::component::ComponentRef;
    use regecs::component_pool;
    use regecs::scene::state::System;
    use regecs::system::Update;

    pub struct Test {
        pub value: i32,
    }

    impl Component for Test {
        type List = BasicComponentList<Test>;
    }

    pub struct Test2 {
        pub value2: i32,
    }

    impl Component for Test2 {
        type List = BasicComponentList<Test2>;
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
        type List = GroupComponentList<u32, ComplexComponent>;
    }

    component_pool! {
        /// A test component pool.
        #[derive(Default, Clear)]
        pub pool TestComponentManager {
            tests: Test,
            test2s: Test2,
            complexes: ComplexComponent,
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

    impl<I: regecs::scene::Configuration> Update<System<I>> for ComplexSystem
    where
        I::Pool: ComponentPool<ComplexComponent>,
    {
        fn update(&mut self, ctx: &mut System<I>, _: &I::AppState) {
            println!("____");
            while let Some((component, new_order)) = self.events.pop() {
                ctx.pool
                    .store_mut()
                    .unchecked_list_mut()
                    .update_group(component.index, new_order);
            }
            for (i, v) in ctx.pool.store_mut().iter_mut() {
                if v.last_order != v.order {
                    // Record new events
                    self.events.push((ComponentRef::new(i), v.order));
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

impl<I: regecs::scene::Configuration<AppState = i32>> Update<state::System<I>> for MySystem
where
    I::Pool: ComponentPool<components::Test> + ComponentPool<components::Test2>,
{
    fn update(&mut self, ctx: &mut state::System<I>, state: &I::AppState) {
        let test: ComponentRef<components::Test> = ComponentRef::new(0);
        let test2: ComponentRef<components::Test2> = ComponentRef::new(0);
        ctx.pool.store_mut()[test].value = 12;
        ctx.pool.store_mut()[test2].value2 = 42;
        assert_eq!(ctx.pool.store()[test2].value2, 42);
        assert_eq!(*state, 42);
    }
}

#[derive(Default)]
struct MySystem2 {}

#[derive(Default, Update)]
#[for_context(state::System<Interface>)]
pub struct TestSystemManager {
    my: MySystem,
    complex: ComplexSystem,
    #[no_update]
    my2: MySystem2,
}

#[derive(Class)]
pub struct Test;

impl Object<Interface> for Test {
    fn on_event(&mut self, _: &mut state::Object<Interface>, _: &i32, _: &Event<()>) {
        todo!()
    }

    fn on_remove(&mut self, _: &mut state::Object<Interface>, _: &i32) {
        todo!()
    }

    fn on_update(&mut self, _: &mut state::Object<Interface>, _: &i32) {
        todo!()
    }
}

impl regecs::object::New<Interface> for Test {
    type Arguments = i32;

    fn new(_: &mut state::Object<Interface>, _: &i32, _: ObjectRef, _: Self::Arguments) -> Self {
        Self {}
    }
}

regecs::import_object!(pub Null<Interface>(regecs::object::builder::NullObject));

regecs::register_objects! {
    /// The root factory for all objects of this test.
    pub builder ObjectBuilder for object RootObject<Interface> {
        /// A test object.
        Test: Test,
        /// A null object.
        Null: Null,
    }
}

pub struct Interface;
impl regecs::scene::Configuration for Interface {
    type Event = ();
    type AppState = i32;
    type Pool = components::TestComponentManager;
    type SystemManager = TestSystemManager;
    type Builder = ObjectBuilder;

    fn into_inner(self) -> (Self::Pool, Self::SystemManager) {
        (
            components::TestComponentManager::default(),
            TestSystemManager::default(),
        )
    }
}

fn main() {
    let mut sc = Scene::new(Interface);
    let ctx = 42;
    let mgr = sc.component_manager_mut();
    //let mut entity = Entity::new(mgr, 0);
    //let test = entity.add_attach(components::Test { value: 12 });
    let test = mgr
        .store_mut()
        .add_attach(0 as EntityIndex, components::Test { value: 12 });
    mgr.store_mut()[test].value = 1;
    let test1 = mgr.store_mut().add(components::Test { value: 0 });
    let test2 = mgr.store_mut().add(components::Test2 { value2: 0 });
    mgr.store_mut().add(ComplexComponent::new(2, 3));
    mgr.store_mut().add(ComplexComponent::new(1, 1));
    mgr.store_mut().add(ComplexComponent::new(2, 4));
    mgr.store_mut().add(ComplexComponent::new(1, 2));
    let systems = sc.system_manager_mut();
    systems.my.val = 42;
    sc.update(&ctx);
    sc.update(&ctx);
    let (mut mgr, _) = sc.into_inner();
    assert_eq!(mgr.store()[test].value, 12);
    assert_eq!(mgr.store()[test2].value2, 42);
    mgr.store_mut().remove(test);
    mgr.store_mut().remove(test2);
    let sfdk =
        <components::TestComponentManager as ComponentPool<components::Test>>::store(&mgr).len();
    let fh =
        <components::TestComponentManager as ComponentPool<components::Test2>>::store(&mgr).len();
    assert_eq!(sfdk, 1);
    assert_eq!(fh, 0);
    mgr.store_mut().remove(test1);
    let test =
        <components::TestComponentManager as ComponentPool<components::Test>>::store(&mgr).len();
    assert_eq!(test, 0);
}

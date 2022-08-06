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
use regecs::{Create, entity::{Entity, EntityPart}, scene::Scene};
use regecs::component::ComponentRef;
use regecs::component::pool::ComponentManager;
use regecs::component::pool::ComponentPool;
use regecs::event::Event;
use regecs::object::{Context as _, Object, ObjectRef};
use regecs::scene::{EventInfo, ObjectContext, SystemContext};
use regecs::system::{Context as _, Update};

use crate::components::{ComplexComponent};

mod components
{
    use regecs::{
        component::{
            pool::{BasicComponentPool, GroupComponentPool},
            Component
        }
    };
    use regecs::component::{Clear, ComponentRef, Pool};
    use regecs::component::pool::{Attachments, ComponentManager, Iter};
    use regecs::object::ObjectRef;
    use regecs::system::Update;

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

    #[derive(Default)]
    pub struct TestComponentManager
    {
        tests: Pool<Test>,
        test2s: Pool<Test2>,
        complexes: Pool<ComplexComponent>
    }

    regecs::impl_component_manager!(TestComponentManager { (tests: Test) (test2s: Test2) (complexes: ComplexComponent) });

    // TODO: Implement a derive proc macro for Clear
    impl Clear for TestComponentManager
    {
        fn clear(&mut self, entity: ObjectRef)
        {
            self.tests.clear(entity);
            self.test2s.clear(entity);
            self.complexes.clear(entity);
        }
    }

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

    impl<C: regecs::system::Context> Update<C> for ComplexSystem
    where
        C::ComponentManager: ComponentManager<ComplexComponent>
    {
        fn update(&mut self, ctx: &mut C, _: &C::AppState)
        {
            println!("____");
            while let Some((component, new_order)) = self.events.pop() {
                ctx.components_mut().pool_mut().update_group(component, new_order);
            }
            for (i, v) in ctx.components_mut().pool_mut().iter_mut() {
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

impl<C: regecs::system::Context<AppState = i32>> Update<C> for MySystem
where
    C::ComponentManager:
        ComponentManager<components::Test> + ComponentManager<components::Test2>
{
    fn update(&mut self, ctx: &mut C, state: &C::AppState)
    {
        let test: ComponentRef<components::Test> = ComponentRef::new(0);
        let test2: ComponentRef<components::Test2> = ComponentRef::new(0);
        ctx.components_mut().get_mut(test).value = 12;
        ctx.components_mut().get_mut(test2).value2 = 42;
        assert_eq!(
            ctx.components().get(test2).value2,
            42
        );
        assert_eq!(*state, 42);
    }
}

#[derive(Default)]
struct MySystem2 {}

#[derive(Default)]
pub struct TestSystemManager
{
    my: MySystem,
    complex: ComplexSystem,
    my2: MySystem2
}

pub struct Test;

impl Object<Ctx1> for Test {
    fn on_event(&mut self, ctx: &mut Ctx1, state: &i32, event: &Event<()>) {
        todo!()
    }

    fn on_remove(&mut self, ctx: &mut Ctx1, state: &i32) {
        todo!()
    }

    fn on_update(&mut self, ctx: &mut Ctx1, state: &i32) {
        todo!()
    }

    fn class(&self) -> &str {
        todo!()
    }
}

impl regecs::object::New<Ctx1> for Test {
    type Arguments = ();

    fn new(_: &mut Ctx1, _: &i32, _: ObjectRef, _: Self::Arguments) -> Self {
        Self {}
    }
}

use regecs_codegen::Object;

#[derive(Object)]
#[context(Ctx1)]
pub struct NullObject(regecs::object::factory::NullObject);

#[derive(Object)]
#[context(Ctx1)]
pub enum RootObject1 {
    Null(NullObject),
    Test(Test)
}

type Ctx = SystemContext<Interface>;
type Ctx1 = ObjectContext<Interface>;

regecs::test_macro!{pub RootFactory for RootObject where context = Ctx1 [
    (Test: Test)
]}

regecs::register_objects!(
    /// The root factory for all objects of this test.
    pub RootFactory {
        context = Ctx1;
        /// The root object enumeration to allow expanding of dynamic dispatches into static dispatches.
        object = RootObject;
        map = [(Test: Test)];
    }
);

pub struct Interface;
impl regecs::scene::Interface for Interface {
    type Event = ();
    type AppState = i32;
    type ComponentManager = components::TestComponentManager;
    type SystemManager = TestSystemManager;
    type Factory = /*regecs::object::factory::NullFactory<Self>*/RootFactory;
}

//TODO: Create a derive macro for Update<T>
impl Update<Ctx> for TestSystemManager
{
    fn update(&mut self, ctx: &mut Ctx, state: &i32)
    {
        self.my.update(ctx, state);
        self.complex.update(ctx, state);
    }
}

fn main()
{
    use regecs::Create;
    let factort = Test::create(());
    let ctx = 42;
    let mut mgr = components::TestComponentManager::default();
    let mut entity = Entity::new(&mut mgr, 0);
    let test = entity.add_attach(components::Test { value: 12 });
    mgr.get_mut(test).value = 1;
    let test1 = mgr.add(components::Test { value: 0 });
    let test2 = mgr.add(components::Test2 { value2: 0 });
    mgr.add(ComplexComponent::new(2, 3));
    mgr.add(ComplexComponent::new(1, 1));
    mgr.add(ComplexComponent::new(2, 4));
    mgr.add(ComplexComponent::new(1, 2));
    let mut systems = TestSystemManager::default();
    systems.my.val = 42;
    let mut sc: Scene<Interface> = Scene::new(mgr, systems);
    sc.update(&ctx);
    sc.update(&ctx);
    mgr = sc.consume();
    assert_eq!(mgr.get(test).value, 12);
    assert_eq!(
        mgr.get(test2).value2,
        42
    );
    mgr.remove(test);
    mgr.remove(test2);
    let sfdk = <components::TestComponentManager as ComponentManager<components::Test>>::pool(&mgr).len();
    let fh = <components::TestComponentManager as ComponentManager<components::Test2>>::pool(&mgr).len();
    assert_eq!(sfdk, 1);
    assert_eq!(fh, 0);
    mgr.remove(test1);
    let test = <components::TestComponentManager as ComponentManager<components::Test>>::pool(&mgr).len();
    assert_eq!(test, 0);
}

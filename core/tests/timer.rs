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

use regecs::component::Clear;
use regecs::component_pool;
use regecs::entity::EntityIndex;
use regecs::scene::Scene;
use std::thread::sleep;
use std::time::Duration;

const EXPECTED_TICK_COUNT: usize = 4;

mod components {
    use regecs::component::list::BasicComponentList;
    use regecs::component::{Component, ComponentRef};
    use regecs::object::ObjectRef;
    use std::time::{Duration, Instant};

    pub struct Timer<E> {
        pub event: fn(ComponentRef<Timer<E>>) -> E,
        pub target: Option<ObjectRef>,
        pub interval: Duration,
        pub last_trigger_time: Instant,
        pub tick_counter: usize,
    }

    impl<E> Timer<E> {
        pub fn with_target(
            event: fn(ComponentRef<Timer<E>>) -> E,
            interval: Duration,
            target: ObjectRef,
        ) -> Self {
            Self {
                event,
                interval,
                target: Some(target),
                last_trigger_time: Instant::now(),
                tick_counter: 0,
            }
        }
    }

    impl<E> Component for Timer<E> {
        type List = BasicComponentList<Timer<E>>;
    }
}

mod systems {
    use regecs::component::list::Iter;
    use regecs::component::{ComponentPool, ComponentRef};
    use regecs::event::Builder;
    use regecs::scene::state::System;
    use regecs::scene::Configuration;
    use regecs::system::Update;
    use std::time::Instant;

    #[derive(Default)]
    pub struct TimerManager;
    impl<C: Configuration> Update<System<C>> for TimerManager
    where
        C::Pool: ComponentPool<super::components::Timer<C::Event>>,
    {
        fn update(&mut self, ctx: &mut System<C>, _: &C::AppState) {
            let now = Instant::now();
            for (index, component) in ctx.pool.store_mut().iter_mut() {
                if now - component.last_trigger_time >= component.interval {
                    println!("{:?}", now - component.last_trigger_time);
                    component.last_trigger_time = now;
                    component.tick_counter += 1;
                    let event = (component.event)(ComponentRef::new(index));
                    match component.target {
                        None => ctx.event_manager.send(Builder::new(event)),
                        Some(target) => target.send(&mut ctx.event_manager, None, event),
                    }
                }
            }
        }
    }
}

mod objects {
    use crate::{Config, EXPECTED_TICK_COUNT};
    use regecs::component::{ComponentPool, ComponentRef};
    use regecs::object::builder::Builder;
    use regecs::object::{Class, Object, ObjectRef};
    use regecs::scene::event::Notify;
    use std::time::Duration;

    pub enum Event {
        Timer(ComponentRef<super::components::Timer<Event>>),
    }

    pub struct TimerTest {
        this: ObjectRef,
    }

    impl Class for TimerTest {
        fn class(&self) -> &str {
            "TimerTest"
        }
    }

    impl Object<regecs::scene::state::Object<Config>> for TimerTest {
        fn on_event(
            &mut self,
            ctx: &mut regecs::scene::state::Object<Config>,
            _: &(),
            event: &regecs::event::Event<Event>,
        ) {
            match event.data() {
                Event::Timer(r) => {
                    let comp = &ctx.common.pool.store()[r];
                    if comp.tick_counter >= EXPECTED_TICK_COUNT {
                        ctx.common.pool.store_mut().remove(r);
                    }
                    if ctx.common.pool.store().attachments(self.this).count() <= 0 {
                        self.this.remove(&mut ctx.common.scene, Notify::None);
                    }
                },
            }
        }
    }

    pub struct TimerTestBuilder;

    impl Builder<regecs::scene::state::Object<Config>> for TimerTestBuilder {
        type Object = TimerTest;

        fn build(
            self,
            ctx: &mut regecs::scene::state::Object<Config>,
            _: &(),
            this: ObjectRef,
        ) -> Self::Object {
            ctx.common.pool.store_mut().add_attach(
                this,
                super::components::Timer::with_target(Event::Timer, Duration::from_secs(1), this),
            );
            ctx.common.pool.store_mut().add_attach(
                this,
                super::components::Timer::with_target(
                    Event::Timer,
                    Duration::from_millis(500),
                    this,
                ),
            );
            ctx.common.pool.store_mut().add_attach(
                this,
                super::components::Timer::with_target(Event::Timer, Duration::from_millis(1), this),
            );
            ctx.common.pool.store_mut().add_attach(
                this,
                super::components::Timer::with_target(Event::Timer, Duration::from_millis(5), this),
            );
            ctx.common.pool.store_mut().add_attach(
                this,
                super::components::Timer::with_target(
                    Event::Timer,
                    Duration::from_millis(10),
                    this,
                ),
            );
            TimerTest { this }
        }
    }
}

component_pool! {
    #[derive(Default)]
    pool TimerPool {
        timers: components::Timer<objects::Event>,
    }
}

impl Clear for TimerPool {
    fn clear(&mut self, entity: EntityIndex) {
        self.timers.clear(entity);
    }
}

struct Config;

impl regecs::scene::Configuration for Config {
    type Event = objects::Event;
    type AppState = ();
    type Pool = TimerPool;
    type SystemManager = systems::TimerManager;
    type Builder = objects::TimerTestBuilder;

    fn into_inner(self) -> (Self::Pool, Self::SystemManager) {
        (Self::Pool::default(), Self::SystemManager::default())
    }
}

#[test]
fn timer_main() {
    let mut scene = Scene::new(Config);
    scene.spawn_object(objects::TimerTestBuilder);
    scene.update(&());
    while scene.state().common.tree.len() > 0 {
        sleep(Duration::from_micros(1));
        scene.update(&());
    }
}

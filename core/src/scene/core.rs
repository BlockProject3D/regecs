// Copyright (c) 2022, BlockProject 3D
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

use std::collections::HashSet;
use std::marker::PhantomData;
use crate::event::{Builder, Event, EventManager};
use crate::object::{Context, Object, ObjectRef, Storage, Tree, Factory};
//use crate::object::factory::Function;
use crate::scene::{Interface, ObjectContext};
use crate::scene::state::{State, Common};
use crate::system::Update;

/// Represents a scene, provides storage for systems and objects
pub struct Scene<I: Interface> {
    state: ObjectContext<I>,
    objects: Storage<ObjectContext<I>>,
    updatable: HashSet<ObjectRef>,
    init_updatable: HashSet<ObjectRef>
}

impl<I: Interface> Scene<I>
{
    pub fn new(component_manager: I::ComponentManager, systems: I::SystemManager) -> Scene<I> {
        return Scene {
            state: State {
                common: Common {
                    component_manager,
                    event_manager: EventManager::new(),
                    system_event_manager: EventManager::new(),
                    tree: Tree::new()
                },
                systems,
                useless: PhantomData::default()
            },
            objects: Storage::new(),
            updatable: HashSet::new(),
            init_updatable: HashSet::new()
        };
    }

    fn object_event_call(&mut self, state: &I::AppState, obj_ref: ObjectRef, event: &Event<I::Event>)
    {
        if !self.state.common.tree.is_enabled(obj_ref) {
            //Disabled objects are not allowed to handle any event
            return;
        }
        let obj = &mut self.objects[obj_ref];
        obj.on_event(&mut self.state, state, &event);
    }

    fn handle_system_event(&mut self, state: &I::AppState, ev: Event<super::event::Event<ObjectContext<I>>>)
    {
        let sender = ev.sender();
        let target = ev.target();
        let inner = ev.into_inner();
        match inner.ty {
            super::event::Type::EnableObject(flag) => {
                let target = target.expect("No target given to EnableObject");
                self.state.common.tree.set_enabled(target, flag);
                if !flag {
                    self.updatable.remove(&target);
                } else if flag && self.init_updatable.contains(&target) {
                    self.updatable.insert(target);
                }
            },
            super::event::Type::SpawnObject(factory) => {
                let updatable = factory.can_update_object();
                let (obj_ref, obj) = self.objects.insert(|this_ref| Box::new(factory.spawn(&mut self.state, state, this_ref)));
                self.state.common.tree.insert(obj_ref, obj.class());
                if updatable {
                    self.updatable.insert(obj_ref);
                    self.init_updatable.insert(obj_ref);
                }
            },
            super::event::Type::RemoveObject => {
                let target = target.expect("No target given to RemoveObject");
                self.objects[target].on_remove(&mut self.state, state);
                self.state.common.tree.remove(target, self.objects[target].class());
                self.objects.destroy(target);
            }
        };
        if inner.notify {
            match sender {
                None => {
                    //TODO: Broadcast notification event
                }
                Some(_target) => {
                    //TODO: Send notification event to `target`
                }
            }
        }
    }

    pub fn update(&mut self, state: &I::AppState)
    {
        self.state.systems.update(&mut self.state.common, state);
        while let Some(ev) = self.state.common.system_event_manager.poll() {
            self.handle_system_event(state, ev);
        }
        for obj in &self.updatable {
            self.objects[*obj].on_update(&mut self.state, state);
        }
        while let Some(event) = self.state.common.event_manager.poll() {
            if let Some(obj_ref) = event.target() {
                self.object_event_call(state, obj_ref, &event);
            } else {
                for (obj_ref, obj) in self.objects.objects().enumerate() {
                    if let Some(o) = obj.as_mut() {
                        if self.state.common.tree.is_enabled(obj_ref as ObjectRef) {
                            o.on_event(&mut self.state, state, &event);
                        }
                    }
                }
            }
        }
    }

    pub fn spawn_object(&mut self, factory: I::Factory)
    {
        let ev = super::event::Event {
            notify: false,
            ty: super::event::Type::SpawnObject(factory)
        };
        self.state.common.system_event_manager.send(Builder::new(ev));
    }

    pub fn state_mut(&mut self) -> &mut impl Context {
        &mut self.state
    }

    pub fn state(&self) -> & impl Context {
        &self.state
    }

    //TODO: Allow turning the scene into it's system manager and component manager

    pub fn consume(self) -> I::ComponentManager
    {
        return self.state.common.component_manager;
    }
}

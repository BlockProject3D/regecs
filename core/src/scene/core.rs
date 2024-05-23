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

use crate::component::Clear;
use crate::event::{Builder, Event, EventManager};
use crate::object::{builder::Builder as ObjectBuilder, Class, Object, ObjectRef, Storage, Tree};
use crate::scene::Interface;
use crate::system::Update;
use std::collections::HashSet;
use crate::scene::state;

/// Represents a scene, provides storage for systems and objects
pub struct Scene<I: Interface> {
    state: state::Object<I>,
    objects: Storage<I>,
    updatable: HashSet<ObjectRef>,
}

impl<I: Interface> Scene<I> {
    pub fn new(interface: I) -> Scene<I> {
        let (component_manager, systems) = interface.into_inner();
        return Scene {
            state: state::Object {
                common: state::System {
                    pool: component_manager,
                    event_manager: EventManager::new(),
                    scene: EventManager::new(),
                    tree: Tree::new(),
                },
                systems
            },
            objects: Storage::new(),
            updatable: HashSet::new(),
        };
    }

    fn object_event_call(
        &mut self,
        state: &I::AppState,
        obj_ref: ObjectRef,
        event: &Event<I::Event>,
    ) {
        if !self.state.common.tree.can_handle_events(obj_ref) {
            //Disabled objects are not allowed to handle any event
            return;
        }
        let obj = &mut self.objects[obj_ref];
        obj.on_event(&mut self.state, state, &event);
    }

    fn handle_system_event(
        &mut self,
        state: &I::AppState,
        ev: Event<super::event::Event<I>>,
    ) {
        let sender = ev.sender();
        let target = ev.target();
        let inner = ev.into_inner();
        match inner.ty {
            super::event::Type::EnableObject(flag) => {
                let target = target.expect("No target given to EnableObject");
                self.state.common.tree.set_enabled(target, flag);
                if unsafe {
                    self.state
                        .common
                        .tree
                        .get_flags(target)
                        .unwrap_unchecked()
                        .is_updatable()
                } {
                    if flag {
                        self.updatable.insert(target);
                    } else {
                        self.updatable.remove(&target);
                    }
                }
            },
            super::event::Type::SpawnObject(builder) => {
                let (obj_ref, obj) = self
                    .objects
                    .insert(|obj_ref| Box::new(builder.build(&mut self.state, state, obj_ref)));
                let flags = obj.flags();
                if flags.is_updatable() {
                    self.updatable.insert(obj_ref);
                }
                self.state.common.tree.insert(obj_ref, flags, obj.class());
            },
            super::event::Type::RemoveObject => {
                let target = target.expect("No target given to RemoveObject");
                self.state.common.pool.clear(target.into_raw());
                self.objects[target].on_remove(&mut self.state, state);
                self.state
                    .common
                    .tree
                    .remove(target, self.objects[target].class());
                self.objects.destroy(target);
            },
        };
        if inner.notify {
            match sender {
                None => {
                    //TODO: Broadcast notification event
                },
                Some(_target) => {
                    //TODO: Send notification event to `target`
                },
            }
        }
    }

    pub fn update(&mut self, state: &I::AppState) {
        self.state.systems.update(&mut self.state.common, state);
        while let Some(ev) = self.state.common.scene.poll() {
            self.handle_system_event(state, ev);
        }
        for obj in &self.updatable {
            self.objects[*obj].on_update(&mut self.state, state);
        }
        while let Some(event) = self.state.common.event_manager.poll() {
            if let Some(obj_ref) = event.target() {
                self.object_event_call(state, obj_ref, &event);
            } else {
                for (obj_ref, obj) in self.objects.iter_mut().enumerate() {
                    if let Some(o) = obj.as_mut() {
                        if self
                            .state
                            .common
                            .tree
                            .is_enabled(unsafe { ObjectRef::from_raw(obj_ref as _) })
                        {
                            o.on_event(&mut self.state, state, &event);
                        }
                    }
                }
            }
        }
    }

    pub fn spawn_object(&mut self, builder: I::Builder) {
        let ev = super::event::Event {
            notify: false,
            ty: super::event::Type::SpawnObject(builder),
        };
        self.state
            .common
            .scene
            .send(Builder::new(ev));
    }

    pub fn component_manager_mut(&mut self) -> &mut I::Pool {
        &mut self.state.common.pool
    }

    pub fn system_manager_mut(&mut self) -> &mut I::SystemManager {
        &mut self.state.systems
    }

    pub fn component_manager(&self) -> &I::Pool {
        &self.state.common.pool
    }

    pub fn system_manager(&self) -> &I::SystemManager {
        &self.state.systems
    }

    //TODO: Allow turning the scene into it's system manager and component manager

    pub fn into_inner(self) -> (I::Pool, I::SystemManager) {
        return (self.state.common.pool, self.state.systems);
    }
}

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
use crate::component::Clear;
use crate::event::{Builder, Event, EventManager};
use crate::object::{Factory, ObjectRef, ObjectStorage};
use crate::scene::{ObjectContext, SystemContext};
use crate::scene::state::{ObjectState, SystemState};
use crate::system::Update;

//TODO: Wrap all this nightmare of generics in another trait (maybe SceneContext) and export clearer names in the trait
/// Represents a scene, provides storage for systems and objects
pub struct Scene<SM: Update<SystemContext<SM, CM, E, S>>, CM: Clear, E, S> {
    scene1: ObjectContext<SM, CM, E, S>,
    objects: ObjectStorage<ObjectContext<SM, CM, E, S>>,
    updatable: HashSet<ObjectRef>,
    init_updatable: HashSet<ObjectRef>
}

impl<SM: Update<SystemContext<SM, CM, E, S>>, CM: Clear, E, S> Scene<SM, CM, E, S>
{
    pub fn new(component_manager: CM, systems: SM) -> Scene<SM, CM, E, S>
    {
        let (objects, tree) = ObjectStorage::new();
        return Scene {
            scene1: ObjectState {
                common: SystemState {
                    component_manager,
                    event_manager: EventManager::new(),
                    system_event_manager: EventManager::new(),
                    tree
                },
                systems
            },
            objects,
            updatable: HashSet::new(),
            init_updatable: HashSet::new()
        };
    }

    fn object_event_call(&mut self, state: &S, obj_ref: ObjectRef, event: &Event<E>)
    {
        if !self.scene1.common.tree.is_enabled(obj_ref) {
            //Disabled objects are not allowed to handle any event
            return;
        }
        let obj = &mut self.objects[obj_ref];
        /*let res = */obj.on_event(&mut self.scene1, state, &event);
        /*if event.tracking {
            self.scene1
                .common
                .event_manager
                .queue_response(event.handle, res);
        }*/
    }

    fn handle_system_event(&mut self, state: &S, ev: Event<super::event::Event<ObjectContext<SM, CM, E, S>>>)
    {
        let sender = ev.sender();
        let target = ev.target();
        let inner = ev.into_inner();
        match inner.ty {
            super::event::Type::EnableObject(flag) => {
                let target = target.expect("No target given to EnableObject");
                self.objects
                    .set_enabled(&mut self.scene1.common.tree, target, flag);
                if !flag {
                    self.updatable.remove(&target);
                } else if flag && self.init_updatable.contains(&target) {
                    self.updatable.insert(target);
                }
            },
            super::event::Type::SpawnObject(obj) => {
                let updatable = obj.updates();
                let (obj_ref, obj) = self.objects.insert(|this_ref| obj.invoke(&mut self.scene1, state, this_ref));
                self.scene1.common.tree.insert(obj_ref, obj.class());
                //let updatable = obj.on_init(&mut self.scene1, state);
                if updatable {
                    self.updatable.insert(obj_ref);
                    self.init_updatable.insert(obj_ref);
                }
            },
            super::event::Type::RemoveObject => {
                let target = target.expect("No target given to RemoveObject");
                self.objects[target].on_remove(&mut self.scene1, state);
                self.scene1.common.tree.remove(target, self.objects[target].class());
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

    pub fn update(&mut self, state: &S)
    {
        self.scene1.systems.update(&mut self.scene1.common, state);
        while let Some(ev) = self.scene1.common.system_event_manager.poll() {
            self.handle_system_event(state, ev);
        }
        for obj in &self.updatable {
            self.objects[*obj].on_update(&mut self.scene1, state);
        }
        while let Some(event) = self.scene1.common.event_manager.poll() {
            if let Some(obj_ref) = event.target() {
                self.object_event_call(state, obj_ref, &event);
            } else {
                for (obj_ref, obj) in self.objects.objects().enumerate() {
                    if let Some(o) = obj.as_mut() {
                        if self.scene1.common.tree.is_enabled(obj_ref as ObjectRef) {
                            o.on_event(&mut self.scene1, state, &event);
                        }
                    }
                }
            }
        }
    }

    pub fn spawn_object(&mut self, factory: Factory<ObjectContext<SM, CM, E, S>>)
    {
        let ev = super::event::Event {
            notify: false,
            ty: super::event::Type::SpawnObject(factory)
        };
        self.scene1.common.system_event_manager.send(Builder::new(ev));
    }

    pub fn consume(self) -> CM
    {
        return self.scene1.common.component_manager;
    }
}

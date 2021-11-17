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

//! REGECS scene object

use std::{any::Any, boxed::Box, collections::HashSet};

use crate::{
    component::Clear,
    event::{Event, EventManager, SystemEvent},
    object::{Context, ObjectFactory, ObjectRef, ObjectStorage, ObjectTree},
    system::Update
};

pub struct Common<C: Context>
{
    component_manager: C::ComponentManager,
    event_manager: EventManager<C>,
    tree: ObjectTree
}

impl<C: Context> crate::system::Context for Common<C>
{
    type AppState = C::AppState;
    type ComponentManager = C::ComponentManager;
    type Context = C;

    fn components(&self) -> &Self::ComponentManager
    {
        return &self.component_manager;
    }

    fn components_mut(&mut self) -> &mut Self::ComponentManager
    {
        return &mut self.component_manager;
    }

    fn event_manager(&mut self) -> &mut EventManager<Self::Context>
    {
        return &mut self.event_manager;
    }

    fn objects(&self) -> &ObjectTree
    {
        return &self.tree;
    }
}

pub struct SceneContext<
    State,
    ComponentManager: Clear,
    TSystemManager: Update<Common<Self>>
> {
    common: Common<Self>,
    systems: TSystemManager
}

impl<State, ComponentManager: Clear, TSystemManager: Update<Common<Self>>>
    crate::object::Context for SceneContext<State, ComponentManager, TSystemManager>
{
    type AppState = State;
    type ComponentManager = ComponentManager;
    type SystemContext = Common<Self>;
    type SystemManager = TSystemManager;

    fn components(&self) -> &Self::ComponentManager
    {
        return &self.common.component_manager;
    }

    fn components_mut(&mut self) -> &mut Self::ComponentManager
    {
        return &mut self.common.component_manager;
    }

    fn event_manager(&mut self) -> &mut EventManager<Self>
    {
        return &mut self.common.event_manager;
    }

    fn systems(&self) -> &Self::SystemManager
    {
        return &self.systems;
    }

    fn systems_mut(&mut self) -> &mut Self::SystemManager
    {
        return &mut self.systems;
    }

    fn objects(&self) -> &ObjectTree
    {
        return &self.common.tree;
    }
}

/// Represents a scene, provides storage for systems and objects
pub struct Scene<
    State,
    ComponentManager: Clear,
    TSystemManager: Update<Common<SceneContext<State, ComponentManager, TSystemManager>>>
> {
    scene1: SceneContext<State, ComponentManager, TSystemManager>,
    objects: ObjectStorage<SceneContext<State, ComponentManager, TSystemManager>>,
    updatable: HashSet<ObjectRef>,
    init_updatable: HashSet<ObjectRef>
}

impl<
        State,
        ComponentManager: Clear,
        TSystemManager: Update<Common<SceneContext<State, ComponentManager, TSystemManager>>>
    > Scene<State, ComponentManager, TSystemManager>
{
    pub fn new(
        component_manager: ComponentManager,
        systems: TSystemManager
    ) -> Scene<State, ComponentManager, TSystemManager>
    {
        let (objects, tree) = ObjectStorage::new();
        return Scene {
            scene1: SceneContext {
                common: Common {
                    component_manager,
                    event_manager: EventManager::new(),
                    tree
                },
                systems
            },
            objects,
            updatable: HashSet::new(),
            init_updatable: HashSet::new()
        };
    }

    fn object_event_call(&mut self, state: &State, obj_ref: ObjectRef, event: &Event)
    {
        if !self.scene1.common.tree.is_enabled(obj_ref) {
            //Non enabled objects are not allowed to handle any event
            return;
        }
        let obj = &mut self.objects[obj_ref];
        let res = obj.on_event(&mut self.scene1, state, &event.data, event.sender);
        if event.tracking {
            self.scene1
                .common
                .event_manager
                .queue_response(event.handle, res);
        }
    }

    fn handle_system_event(
        &mut self,
        state: &State,
        ev: SystemEvent<SceneContext<State, ComponentManager, TSystemManager>>
    ) -> Option<Box<dyn Any>>
    {
        return match ev {
            SystemEvent::Enable(obj, flag) => {
                self.objects
                    .set_enabled(&mut self.scene1.common.tree, obj, flag);
                if !flag {
                    self.updatable.remove(&obj);
                } else if flag && self.init_updatable.contains(&obj) {
                    self.updatable.insert(obj);
                }
                None
            },
            SystemEvent::Serialize(obj) => {
                let data = self.objects[obj].serialize(&self.scene1, state);
                if let Some(d) = data {
                    Some(Box::from(d))
                } else {
                    None
                }
            },
            SystemEvent::Deserialize(obj, data) => {
                self.objects[obj].deserialize(&mut self.scene1, state, data);
                None
            },
            SystemEvent::Spawn(obj) => {
                let (obj_ref, obj) = self.objects.insert(&mut self.scene1.common.tree, obj);
                let updatable = obj.on_init(&mut self.scene1, state);
                if updatable {
                    self.updatable.insert(obj_ref);
                    self.init_updatable.insert(obj_ref);
                }
                Some(Box::new(obj_ref))
            },
            SystemEvent::Destroy(target) => {
                self.objects[target].on_remove(&mut self.scene1, state);
                self.objects.destroy(&mut self.scene1.common.tree, target);
                None
            }
        };
    }

    pub fn update(&mut self, state: &State)
    {
        self.scene1.systems.update(&mut self.scene1.common, state);
        while let Some((tracking, handle, ev)) =
            self.scene1.common.event_manager.poll_system_event()
        {
            let res = self.handle_system_event(state, ev);
            if tracking {
                self.scene1.common.event_manager.queue_response(handle, res);
            }
        }
        for obj in &self.updatable {
            self.objects[*obj].on_update(&mut self.scene1, state);
        }
        while let Some(event) = self.scene1.common.event_manager.poll_event() {
            if let Some(obj_ref) = event.target {
                self.object_event_call(state, obj_ref, &event);
            } else {
                for (obj_ref, obj) in self.objects.objects().enumerate() {
                    if let Some(o) = obj.as_mut() {
                        if self.scene1.common.tree.is_enabled(obj_ref as ObjectRef) {
                            let res =
                                o.on_event(&mut self.scene1, state, &event.data, event.sender);
                            if event.tracking {
                                self.scene1
                                    .common
                                    .event_manager
                                    .queue_response(event.handle, res);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn spawn_object(
        &mut self,
        factory: ObjectFactory<SceneContext<State, ComponentManager, TSystemManager>>
    )
    {
        self.scene1
            .common
            .event_manager
            .system(SystemEvent::Spawn(factory), false);
    }

    pub fn consume(self) -> ComponentManager
    {
        return self.scene1.common.component_manager;
    }
}

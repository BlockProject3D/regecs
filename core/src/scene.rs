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

use std::{any::Any, boxed::Box};

use crate::{
    object::{CoreObject, ObjectRef},
    system::System,
};
use crate::event::{EventManager, Event, SystemEvent};
use crate::object::{EventContext, CommonContext};
use std::collections::HashSet;

/// Represents a scene, provides storage for systems and objects
pub struct Scene<TState, TComponentManager>
{
    component_manager: TComponentManager,
    systems: Vec<Box<dyn System<TState, TComponentManager>>>,
    objects: Vec<Option<Box<dyn CoreObject<TState, TComponentManager>>>>,
    updatables: HashSet<ObjectRef>,
    event_manager: EventManager<TState, TComponentManager>,
}

impl<TState, TComponentManager> Scene<TState, TComponentManager>
{
    pub fn new(component_manager: TComponentManager) -> Scene<TState, TComponentManager>
    {
        return Scene {
            component_manager,
            systems: Vec::new(),
            objects: Vec::new(),
            updatables: HashSet::new(),
            event_manager: EventManager::new(),
        };
    }

    fn object_event_call(&mut self, ctx: &mut TState, obj_ref: ObjectRef, event: &Event)
    {
        let obj = self.objects[obj_ref as usize].as_mut().unwrap();
        let res = obj.on_event(EventContext {
            components: &mut self.component_manager,
            event_manager: &mut self.event_manager,
            this: obj_ref,
            sender: event.sender,
            state: ctx,
        }, &event.data);
        if event.tracking {
            self.event_manager.queue_response(event.handle, res);
        }
    }

    fn handle_system_event(&mut self, ctx: &mut TState, ev: SystemEvent<TState, TComponentManager>) -> Option<Box<dyn Any>>
    {
        return match ev {
            SystemEvent::EnableUpdate(obj, flag) => {
                if flag {
                    self.updatables.insert(obj);
                } else {
                    self.updatables.remove(&obj);
                }
                None
            }
            SystemEvent::Serialize(obj) => {
                let o = self.objects[obj as usize].as_mut().unwrap();
                let data = o.serialize(CommonContext {
                    components: &mut self.component_manager,
                    event_manager: &mut self.event_manager,
                    this: obj,
                    state: ctx,
                });
                if let Some(d) = data {
                    Some(Box::from(d))
                } else {
                    None
                }
            }
            SystemEvent::Deserialize(obj, data) => {
                let o = self.objects[obj as usize].as_mut().unwrap();
                o.deserialize(CommonContext {
                    components: &mut self.component_manager,
                    event_manager: &mut self.event_manager,
                    this: obj,
                    state: ctx,
                }, data);
                None
            }
            SystemEvent::Spawn(obj) => {
                Some(Box::new(self.spawn_object_internal(ctx, obj)))
            }
            SystemEvent::Destroy(target) => {
                self.delete_object(ctx, target);
                None
            }
        };
    }

    pub fn update(&mut self, ctx: &mut TState)
    {

        while let Some((tracking, handle, sys)) = self.event_manager.poll_system_event() {
            let res = self.handle_system_event(ctx, sys);
            if tracking {
                self.event_manager.queue_response(handle, res);
            }
        }
        for obj in &self.updatables {
            let o = self.objects[*obj as usize].as_mut().unwrap();
            o.on_update(CommonContext {
                components: &mut self.component_manager,
                event_manager: &mut self.event_manager,
                this: *obj,
                state: ctx,
            });
        }
        while let Some(event) = self.event_manager.poll_event() {
            if let Some(obj) = event.target {
                self.object_event_call(ctx, obj, &event);
            } else {
                for i in 0..self.objects.len() {
                    if self.objects[i].is_some() {
                        self.object_event_call(ctx, i as ObjectRef, &event);
                    }
                }
            }
        }
    }

    fn delete_object(&mut self, ctx: &mut TState, target: ObjectRef)
    {
        {
            let obj = self.objects[target as usize].as_mut().unwrap();
            obj.on_remove(CommonContext {
                components: &mut self.component_manager,
                event_manager: &mut self.event_manager,
                this: target,
                state: ctx,
            });
        }
        self.objects[target as usize] = None;
    }

    fn spawn_object_internal(&mut self, ctx: &mut TState, mut obj: Box<dyn CoreObject<TState, TComponentManager>>) -> ObjectRef
    {
        let empty_slot = {
            let mut id = 0 as usize;
            while id < self.objects.len() && self.objects[id].is_some() {
                id += 1;
            }
            if id == self.objects.len() {
                None
            } else {
                Some(id)
            }
        };

        let mut initpayload = CommonContext {
            components: &mut self.component_manager,
            event_manager: &mut self.event_manager,
            this: 0,
            state: ctx,
        };
        if let Some(slot) = empty_slot {
            initpayload.this = slot as ObjectRef;
            obj.on_init(initpayload);
            self.objects[slot] = Some(obj);
            return slot as ObjectRef;
        } else {
            let id = self.objects.len() as ObjectRef;
            initpayload.this = id;
            obj.on_init(initpayload);
            self.objects.push(Some(obj));
            return id;
        }
    }

    pub fn spawn_object<TObject: CoreObject<TState, TComponentManager> + 'static>(&mut self, obj: TObject)
    {
        self.event_manager.system(SystemEvent::Spawn(Box::from(obj)), false);
    }

    pub fn add_system<TSystem: 'static + System<TState, TComponentManager>>(&mut self, system: TSystem)
    {
        let b = Box::new(system);
        self.systems.push(b);
    }

    pub fn consume(self) -> TComponentManager
    {
        return self.component_manager;
    }
}

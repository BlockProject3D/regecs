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
    event::{Event, EventManager, SystemEvent},
    object::{CoreObject, ObjectRef}
};
use crate::object::{ObjectStorage, ObjectTree};

/// Represents a scene, provides storage for systems and objects
pub struct Scene<TState, TComponentManager>
{
    component_manager: TComponentManager,
    //systems: Vec<Box<dyn System<TState, TComponentManager>>>,
    objects: ObjectStorage<TState, TComponentManager>,
    tree: ObjectTree,
    //objects: Vec<Option<Box<dyn CoreObject<TState, TComponentManager>>>>,
    //updatables: HashSet<ObjectRef>,
    event_manager: EventManager<TState, TComponentManager>
}

impl<TState, TComponentManager> Scene<TState, TComponentManager>
{
    pub fn new(component_manager: TComponentManager) -> Scene<TState, TComponentManager>
    {
        let (objects, tree) = ObjectStorage::new();
        return Scene {
            component_manager,
            //systems: Vec::new(),
            objects,
            tree,
            event_manager: EventManager::new()
        };
    }

    fn object_event_call(ctx: &mut crate::object::Context<TState, TComponentManager>, obj: &mut Box<dyn CoreObject<TState, TComponentManager>>, obj_ref: ObjectRef, event: &Event)
    {
        let res = obj.on_event(ctx, &event.data, event.sender, obj_ref);
        if event.tracking {
            ctx.event_manager.queue_response(event.handle, res);
        }
    }

    pub fn update(&mut self, state: &mut TState)
    {
        let mut ctx = crate::object::Context {
            components: &mut self.component_manager,
            event_manager: &mut self.event_manager,
            tree: &self.tree,
            state
        };
        //Use a single context when running objects to lower CPU time by only issuing one set of mov instructions
        while let Some((tracking, handle, ev)) = ctx.event_manager.poll_system_event() {
            let res: Option<Box<dyn Any>> = match ev {
                SystemEvent::EnableUpdate(obj, flag) => {
                    //self.objects.set_updatable(ctx.tree, obj, flag);
                    None
                },
                SystemEvent::Serialize(obj) => {
                    let data = self.objects[obj].serialize(&mut ctx, obj);
                    if let Some(d) = data {
                        Some(Box::from(d))
                    } else {
                        None
                    }
                },
                SystemEvent::Deserialize(obj, data) => {
                    self.objects[obj].deserialize(&mut ctx, data, obj);
                    None
                },
                SystemEvent::Spawn(obj) => {
                    //let (obj_ref, obj) = self.objects.insert(ctx.tree, obj);
                    //obj.on_init(&mut ctx, obj_ref);
                    //Some(Box::new(obj_ref))
                    None
                },
                SystemEvent::Destroy(target) => {
                    //self.objects[target].on_remove(&mut ctx, target);
                    //self.objects.destroy(ctx.tree, target);
                    None
                }
            };
            if tracking {
                ctx.event_manager.queue_response(handle, res);
            }
        } //Must be slowed down due to rust borrow checker
        for obj in self.tree.get_updatable() {
            self.objects[*obj].on_update(&mut ctx, *obj);
        }
        while let Some(event) = ctx.event_manager.poll_event() {
            if let Some(obj_ref) = event.target {
                Self::object_event_call(&mut ctx, &mut self.objects[obj_ref], obj_ref, &event);
            } else {
                for i in self.tree.get_all() {
                    Self::object_event_call(&mut ctx, &mut self.objects[*i], *i, &event);
                }
            }
        }
    }

    pub fn spawn_object<TObject: CoreObject<TState, TComponentManager> + 'static>(&mut self, obj: TObject)
    {
        self.event_manager.system(SystemEvent::Spawn(Box::from(obj)), false);
    }

    /*pub fn add_system<TSystem: 'static + System<TState, TComponentManager>>(&mut self, system: TSystem)
    {
        let b = Box::new(system);
        self.systems.push(b);
    }*/

    pub fn consume(self) -> TComponentManager
    {
        return self.component_manager;
    }
}

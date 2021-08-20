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

use std::{any::Any, boxed::Box, collections::VecDeque};

use crate::{
    event::EventContext,
    object::{CoreObject, ObjectRef},
    system::System
};

/// Represents a scene, provides storage for systems and objects
pub struct Scene<TState, TComponentManager>
{
    component_manager: TComponentManager,
    systems: Vec<Box<dyn System<TState, TComponentManager>>>,
    objects: Vec<Option<Box<dyn CoreObject<TState, TComponentManager>>>>,
    targeted_event_que: VecDeque<(Option<ObjectRef>, ObjectRef, Box<dyn Any>)>,
    untargeted_event_que: VecDeque<(Option<ObjectRef>, Box<dyn Any>)>
}

impl<TState, TComponentManager> Scene<TState, TComponentManager>
{
    pub fn new(component_manager: TComponentManager) -> Scene<TState, TComponentManager>
    {
        return Scene {
            component_manager: component_manager,
            systems: Vec::new(),
            objects: Vec::new(),
            targeted_event_que: VecDeque::new(),
            untargeted_event_que: VecDeque::new()
        };
    }

    fn object_event_call(
        &mut self,
        ctx: &mut TState,
        obj_ref: ObjectRef,
        event: &Box<dyn Any>,
        sender: Option<ObjectRef>
    )
    {
        let obj = self.objects[obj_ref as usize].as_mut().unwrap();
        let res = obj.on_event(
            event,
            EventContext {
                this: obj_ref,
                sender: sender,
                state: ctx,
                components: &mut self.component_manager
            }
        );
        if let Some(events) = res {
            let (remove_flag, events, spawns) = events.consume();
            if remove_flag {
                self.delete_object(obj_ref);
            }
            for (target, event) in events {
                if let Some(target) = target {
                    self.targeted_event_que.push_back((Some(obj_ref), target, event));
                } else {
                    self.untargeted_event_que.push_back((Some(obj_ref), event));
                }
            }
            for obj in spawns {
                self.spawn_object_internal(obj, Some(obj_ref));
            }
        }
    }

    pub fn update(&mut self, ctx: &mut TState)
    {
        for i in 0..self.systems.len() {
            if let Some(events) = self.systems[i].update(ctx, &mut self.component_manager) {
                for (target, event) in events.consume() {
                    self.targeted_event_que.push_back((None, target, event));
                }
            }
        }
        while let Some((sender, target, event)) = self.targeted_event_que.pop_front() {
            self.object_event_call(ctx, target, &event, sender);
        }
        //We only do one untargeted event per frame, as untargeted events are expensive (they need iteration over ALL existing objects in the scene)
        if let Some((sender, event)) = self.untargeted_event_que.pop_front() {
            for i in 0..self.objects.len() {
                if self.objects[i].is_some() {
                    self.object_event_call(ctx, i as ObjectRef, &event, sender);
                }
            }
        }
    }

    fn delete_object(&mut self, target: ObjectRef)
    {
        {
            let obj = self.objects[target as usize].as_mut().unwrap();
            obj.on_remove(&mut self.component_manager, target);
        }
        self.objects[target as usize] = None;
    }

    fn spawn_object_internal(
        &mut self,
        mut obj: Box<dyn CoreObject<TState, TComponentManager>>,
        spawned_by: Option<ObjectRef>
    )
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

        if let Some(slot) = empty_slot {
            obj.on_init(&mut self.component_manager, slot as ObjectRef, spawned_by);
            self.objects[slot] = Some(obj);
        } else {
            let id = self.objects.len() as ObjectRef;
            obj.on_init(&mut self.component_manager, id, spawned_by);
            self.objects.push(Some(obj));
        }
    }

    pub fn spawn_object<TObject: CoreObject<TState, TComponentManager> + 'static>(&mut self, obj: TObject)
    {
        self.spawn_object_internal(Box::from(obj), None);
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

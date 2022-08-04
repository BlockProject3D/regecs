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

#[macro_export]
macro_rules! impl_component_manager {
    (
        $name: ty { $(($pname: ident : $ptype: ty))* }
    ) => {
        $(
            impl regecs::component::pool::ComponentManager<$ptype> for $name {
                fn pool(&self) -> & <$ptype as regecs::component::Component>::Pool {
                    &self.$pname
                }

                fn pool_mut(&mut self) -> &mut <$ptype as regecs::component::Component>::Pool {
                    &mut self.$pname
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_object_wrap {
    ($name: ty { $(($class_name: ident : $object_type: ty))* }) => {
        $(
            impl regecs::object::factory::Wrap<$name> for $object_type {
                fn wrap(self) -> $name {
                    $name::$class_name(self)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! register_objects {
    (
        $(#[$factory_outer: meta])*
        $visibility: vis $factory_name: ident {
            context = $ctx: ty;
            $(#[$object_outer: meta])*
            object = $object_name: ident;
            map = [$(($class_name: ident : $object_type: ty)),*];
        }
    ) => {
        $(#[$object_outer])*
        $visibility enum $object_name {
            $(
                $class_name($object_type),
            )*
        }

        impl regecs::object::Object<$ctx> for $object_name {
            fn on_event(&mut self, ctx: &mut $ctx, state: &<$ctx as regecs::system::Context>::AppState, event: &regecs::event::Event<<$ctx as regecs::system::Context>::Event>) {
                match self {
                    $($object_name::$class_name(v) => v.on_event(ctx, state, event),)*
                }
            }
            fn on_remove(&mut self, ctx: &mut $ctx, state: &<$ctx as regecs::system::Context>::AppState) {
                match self {
                    $($object_name::$class_name(v) => v.on_remove(ctx, state),)*
                }
            }
            fn on_update(&mut self, ctx: &mut $ctx, state: &<$ctx as regecs::system::Context>::AppState) {
                match self {
                    $($object_name::$class_name(v) => v.on_update(ctx, state),)*
                }
            }
            fn class(&self) -> &str {
                match self {
                    $($object_name::$class_name(v) => v.class(),)*
                }
            }
        }

        $(#[$factory_outer])*
        $visibility enum $factory_name {
            $(
                $class_name(<$object_type as regecs::object::New<$ctx>>::Arguments),
            )*
        }

        impl regecs::object::factory::Factory<$ctx> for $factory_name {
            type Object = $object_name;

            fn spawn(self, ctx: &mut $ctx, state: &<$ctx as regecs::system::Context>::AppState,
                this: ObjectRef) -> Self::Object {
                match self {
                    $($factory_name::$class_name(v) =>
                        $object_name::$class_name(<$object_type as regecs::object::New<$ctx>>::new(
                            ctx, state, this, v
                        ))
                    ,)*
                }
            }

            fn can_update_object(&self) -> bool {
                match self {
                    $($factory_name::$class_name(v) =>
                        <$object_type as regecs::object::New<$ctx>>::will_update(v),)*
                }
            }
        }

        $(
            impl regecs::Create<$factory_name> for $object_type {
                type Arguments = <$object_type as regecs::object::New<$ctx>>::Arguments;
                fn create(args: Self::Arguments) -> $factory_name {
                    $factory_name::$class_name(args)
                }
            }
        )*
    };
}

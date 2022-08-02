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
                fn get(&self) -> & <$ptype as regecs::component::Component>::Pool {
                    &self.$pname
                }

                fn get_mut(&mut self) -> &mut <$ptype as regecs::component::Component>::Pool {
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
        $(#[$outer: meta])*
        $visibility: vis $name: ident ($ctx: ty, $factory: ty) {
            $(
                $(#[$object_outer: meta])*
                $class_name: ident : $object_type: ty
            ),*
        }
    ) => {
        $(#[$outer])*
        $visibility enum $name {
            $(
                $(#[$object_outer])*
                $class_name($object_type),
            )*
        }

        $(
            impl regecs::object::factory::Wrap<$name> for $object_type {
                fn wrap(self) -> $name {
                    $name::$class_name(self)
                }
            }
        )*

        impl regecs::object::Object<$ctx> for $name {
            fn on_event(&mut self, ctx: &mut $ctx, state: &<$ctx as regecs::system::Context>::AppState, event: &regecs::event::Event<<$ctx as regecs::system::Context>::Event>) {
                match self {
                    $($name::$class_name(v) => v.on_event(ctx, state, event),)*
                }
            }
            fn on_remove(&mut self, ctx: &mut $ctx, state: &<$ctx as regecs::system::Context>::AppState) {
                match self {
                    $($name::$class_name(v) => v.on_remove(ctx, state),)*
                }
            }
            fn on_update(&mut self, ctx: &mut $ctx, state: &<$ctx as regecs::system::Context>::AppState) {
                match self {
                    $($name::$class_name(v) => v.on_update(ctx, state),)*
                }
            }
            fn class(&self) -> &str {
                match self {
                    $($name::$class_name(v) => v.class(),)*
                }
            }
        }

        //$(
            impl regecs::object::registry::Registry for $name {
                type Factory = $factory;
                fn get_class_map() -> regecs::object::registry::ClassMap<$factory> {
                    let map = std::collections::HashMap::from([
                        $(
                            (std::stringify!($class_name), <$factory as regecs::object::registry::NewFactory<$ctx, $object_type>>::new_factory()),
                        )*
                    ]);
                    regecs::object::registry::ClassMap::new(map)
                }
            }
        //)?
    };
}

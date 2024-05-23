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

#[macro_export]
macro_rules! component_pool {
    (
        $(#[$outer: meta])*
        $visibility: vis pool $pool_name: ident {
            $(
                $(#[$field_outer: meta])*
                $component_name: ident : $component_type: ty,
            )*
        }
    ) => {
        $(#[$outer])*
        $visibility struct $pool_name {
            $(
                $(#[$field_outer])*
                $component_name: $crate::component::store::ComponentStore<$component_type>,
            )*
        }

        $(
            impl $crate::component::ComponentPool<$component_type> for $pool_name {
                fn store(&self) -> &$crate::component::store::ComponentStore<$component_type> {
                    &self.$component_name
                }

                fn store_mut(&mut self) -> &mut $crate::component::store::ComponentStore<$component_type> {
                    &mut self.$component_name
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! register_objects {
    (
        $(#[$outer: meta])*
        $visibility: vis builder $builder_name: ident for object $object_name: ident<$ctx: ty> {
            $(
                $(#[$field_outer: meta])*
                $class_name: ident : $object_type: ty,
            )*
        }
    ) => {
        $(#[$outer])*
        $visibility enum $object_name {
            $(
                $(#[$field_outer])*
                $class_name($object_type),
            )*
        }

        impl $crate::object::Class for $object_name {
            fn class(&self) -> &str {
                match self {
                    $($object_name::$class_name(v) => v.class(),)*
                }
            }
        }

        impl $crate::object::Object<$ctx> for $object_name {
            fn on_event(&mut self, ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState, event: &$crate::event::Event<<$ctx as $crate::scene::Interface>::Event>) {
                match self {
                    $($object_name::$class_name(v) => v.on_event(ctx, state, event),)*
                }
            }
            fn on_remove(&mut self, ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState) {
                match self {
                    $($object_name::$class_name(v) => v.on_remove(ctx, state),)*
                }
            }
            fn on_update(&mut self, ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState) {
                match self {
                    $($object_name::$class_name(v) => v.on_update(ctx, state),)*
                }
            }
        }

        $(#[$outer])*
        $visibility enum $builder_name {
            $(
                $(#[$field_outer])*
                $class_name(<$object_type as $crate::object::New<$ctx>>::Arguments),
            )*
        }

        impl $crate::object::builder::Builder<$ctx> for $builder_name {
            type Object = $object_name;

            fn build(self, ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState,
                this: $crate::object::ObjectRef) -> Self::Object {
                match self {
                    $($builder_name::$class_name(v) =>
                        $object_name::$class_name(<$object_type as $crate::object::New<$ctx>>::new(
                            ctx, state, this, v
                        ))
                    ,)*
                }
            }
        }

        $(
            impl $crate::object::builder::NewBuilder<$ctx> for $object_type {
                fn new_builder(args: Self::Arguments) -> $builder_name {
                    $builder_name::$class_name(args)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! import_object {
    (
        $(#[$outer: meta])*
        $visibility: vis $object_name: ident<$ctx: ty>($object_type: ty)
    ) => {
        $(#[$outer])*
        $visibility struct $object_name($object_type);

        impl $crate::object::Class for $object_name {
            fn class(&self) -> &str {
                self.0.class()
            }
        }

        impl $crate::object::Object<$ctx> for $object_name {
            fn on_event(&mut self, ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState, event: &$crate::event::Event<<$ctx as $crate::scene::Interface>::Event>) {
                self.0.on_event(ctx, state, event)
            }
            fn on_remove(&mut self, ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState) {
                self.0.on_remove(ctx, state)
            }
            fn on_update(&mut self, ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState) {
                self.0.on_update(ctx, state)
            }
        }

        impl $crate::object::New<$ctx> for $object_name {
            type Arguments = <$object_type as $crate::object::New<$ctx>>::Arguments;

            fn new(ctx: &mut $crate::scene::state::Object<$ctx>, state: &<$ctx as $crate::scene::Interface>::AppState, this: $crate::object::ObjectRef, args: Self::Arguments) -> Self {
                $object_name(<$object_type as $crate::object::New<$ctx>>::new(ctx, state, this, args))
            }
        }
    };
}

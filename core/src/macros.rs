pub use impls::impls;
pub use paste::paste;

#[macro_export]
macro_rules! pool_type {
    ($i: ty) => {
        <$i as regecs::component::Component>::Pool
    };
}

#[macro_export]
macro_rules! build_component_manager {
    ($name: ident { $($component: ident $($suffix: ident)?),* }) => {
        use $crate::macros::paste;
        paste! {
            pub struct [<$name ComponentManager>]
            {
                $(
                    [<pool_$component:snake>]: <$component as regecs::component::Component>::Pool,
                )*
            }

            impl [<$name ComponentManager>]
            {
                pub fn new() -> [<$name ComponentManager>]
                {
                    return [<$name ComponentManager>] {
                        $(
                            [<pool_$component:snake>]: <$component as regecs::component::Component>::Pool::new(),
                        )*
                    };
                }
            }

            $(
                impl regecs::component::ComponentProvider<$component> for [<$name ComponentManager>]
                {
                    fn pool(&self) -> & <$component as regecs::component::Component>::Pool
                    {
                        return &self.[<pool_$component:snake>];
                    }

                    fn pool_mut(&mut self) -> &mut <$component as regecs::component::Component>::Pool
                    {
                        return &mut self.[<pool_$component:snake>];
                    }
                }
            )*

            impl regecs::component::ComponentManager for [<$name ComponentManager>]
            {
                fn clear_components(&mut self, entity: regecs::object::ObjectRef)
                {
                    use regecs::component::AttachmentProvider;
                    macro_rules! attachment_call {
                        ($afsb: ident A) => {
                            self.$afsb.clear(entity);
                        };
                        ($afsb: ident) => {};
                    }
                    $(
                        attachment_call!([<pool_$component:snake>] $($suffix)?);
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! build_system_list {
    ($name: ident ( $tstate: ty, $tcomponents: ty ) { $($system: ident),* }) => {
        use $crate::macros::paste;
        use $crate::macros::impls;
        use $crate::system::SystemProvider;
        use $crate::system::SystemList;
        paste! {
            pub struct [<$name SystemList>]
            {
                $(
                    [<sys_$system:snake>]: $system,
                )*
            }

            $(
                impl SystemProvider<$system> for [<$name SystemList>]
                {
                    fn system(&self) -> & $system
                    {
                        return &self.[<sys_$system:snake>];
                    }

                    fn system_mut(&mut self) -> &mut $system
                    {
                        return &mut self.[<sys_$system:snake>];
                    }
                }
            )*

            pub type [<$name SystemCtx>] = $crate::scene::Common<$crate::scene::SceneContext<$tstate, $tcomponents, [<$name SystemList>]>>;
            pub type [<$name Ctx>] = $crate::scene::SceneContext<$tstate, $tcomponents, [<$name SystemList>]>;

            impl SystemList<[<$name SystemCtx>]> for [<$name SystemList>]
            {
                fn update(&mut self, ctx: & [<$name SystemCtx>], state: & $tstate)
                {
                    $(
                        if <$system as System<[<$name SystemCtx>]>>::UPDATABLE {
                            self.[<sys_$system:snake>].update(ctx, state);
                        }
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! object_not_serializable {
    ($tcontext: ty, $object: ty) => {
        impl Serializable<$tcontext> for $object
        {
            fn serialize(&self, _: & $tcontext, _: & <$tcontext as Context>::AppState) -> Option<bpx::sd::Object>
            {
                return None;
            }

            fn deserialize(&mut self, _: & $tcontext, _: & <$tcontext as Context>::AppState, _: bpx::sd::Object);
            {
            }
        }
    };
}

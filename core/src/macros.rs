pub use impls::impls;
pub use paste::paste;

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
macro_rules! build_component_manager1 {
    (
        $(#[$outer:meta])*
        $access: ident $name: ident
        {
            $(
                $(#[$pouter:meta])*
                $(($poption: ident))? $pname: ident : $ptype: ty
            ),*
        }
        $({into ($($types: ty),*) => ($($fields: ident),*)})*
    ) => {
        $(#[$outer])*
        $access struct $name
        {
            $(
                $(#[$pouter])*
                $pname: <$ptype as regecs::component::Component>::Pool
            ),*
        }

        $(
            impl ComponentProvider<$ptype> for $name
            {
                fn pool(&self) -> & <$ptype as regecs::component::Component>::Pool
                {
                    return &self.$pname;
                }

                fn pool_mut(&mut self) -> &mut <$ptype as regecs::component::Component>::Pool
                {
                    return &mut self.$pname;
                }
            }
        )*

        impl regecs::component::ComponentManager for $name
        {
            fn clear_components(&mut self, entity: regecs::object::ObjectRef)
            {
                use regecs::component::AttachmentProvider;
                macro_rules! attachment_call {
                    (attachments $afsb: ident) => {
                        self.$afsb.clear(entity);
                    };
                    ($afsb: ident) => {};
                }
                $(attachment_call!($($poption)? $pname);)*
            }
        }

        $(
            impl Into<($(<$types as regecs::component::Component>::Pool),*)> for $name
            {
                fn into(self) -> ($(<$types as regecs::component::Component>::Pool),*)
                {
                    return ($(self.$fields),*);
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! build_system_list1 {
    (
        $(#[$outer:meta])*
        $access: ident $name: ident < $tstate: ty, $tcomponents: ty >
        {
            $(
                $(#[$pouter:meta])*
                $(($poption: ident))? $pname: ident : $ptype: ty
            ),*
        }
        $({into ($($types: ty),*) => ($($fields: ident),*)})*
    ) => {
        $(#[$outer])*
        $access struct $name
        {
            $(
                $(#[$pouter])*
                $pname: $ptype
            ),*
        }

        use $crate::system::SystemProvider;
        $(
            impl SystemProvider<$ptype> for $name
            {
                fn system(&self) -> & $ptype
                {
                    return &self.$pname;
                }

                fn system_mut(&mut self) -> &mut $ptype
                {
                    return &mut self.$pname;
                }
            }
        )*

        use $crate::system::SystemList;
        use $crate::scene::Common;
        use $crate::scene::SceneContext;
        impl SystemList<Common<SceneContext<$tstate, $tcomponents, $name>>> for $name
        {
            fn update(&mut self, ctx: &mut Common<SceneContext<$tstate, $tcomponents, $name>>, state: & $tstate)
            {
                use regecs::system::Updatable;
                macro_rules! update_call {
                    (updates $afsb: ident) => {
                        self.$afsb.update(ctx, state);
                    };
                    ($afsb: ident) => {};
                }
                $(update_call!($($poption)? $pname);)*
            }
        }

        $(
            impl Into<($($types),*)> for $name
            {
                fn into(self) -> ($($types),*)
                {
                    return ($(self.$fields),*);
                }
            }
        )*
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
                fn update(&mut self, ctx: &mut [<$name SystemCtx>], state: & $tstate)
                {
                    $(
                        self.[<sys_$system:snake>].update(ctx, state);
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

pub use build_component_manager1;
pub use build_system_list1;

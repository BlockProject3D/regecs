#[macro_export]
macro_rules! build_component_manager {
    (
        $(#[$outer:meta])*
        $access: vis $name: ident
        {
            $(
                $(#[$pouter:meta])*
                $(($poption: ident))? $pname: ident : $ptype: ty
            ),*
        }
        $(into ($($types: ty),*) => ($($fields: ident),*))*
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
            impl regecs::component::pool::ComponentManager<$ptype> for $name
            {
                fn get(&self) -> & <$ptype as regecs::component::Component>::Pool
                {
                    return &self.$pname;
                }

                fn get_mut(&mut self) -> &mut <$ptype as regecs::component::Component>::Pool
                {
                    return &mut self.$pname;
                }
            }
        )*

        impl regecs::component::Clear for $name
        {
            fn clear(&mut self, entity: regecs::object::ObjectRef)
            {
                macro_rules! attachment_call {
                    (attachments $afsb: ident $afsb1: ty) => {
                        <<$afsb1 as regecs::component::Component>::Pool as regecs::component::pool::Attachments<$afsb1>>::clear(&mut self.$afsb, entity);
                    };
                    ($afsb: ident $afsb1: ty) => {};
                }
                $(attachment_call!($($poption)? $pname $ptype);)*
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
macro_rules! build_system_manager {
    (
        $(#[$outer:meta])*
        $access: vis $name: ident < $tstate: ty, $tcomponents: ty >
        {
            $(
                $(#[$pouter:meta])*
                $(($poption: ident))? $pname: ident : $ptype: ty
            ),*
        }
        $(into ($($types: ty),*) => ($($fields: ident),*))*

        $(
            $(#[$ctxouter:meta])*
            context $ctxname: ident
        )?
    ) => {
        $(#[$outer])*
        $access struct $name
        {
            $(
                $(#[$pouter])*
                $pname: $ptype
            ),*
        }

        $(
            impl $crate::system::SystemProvider<$ptype> for $name
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

        impl $crate::system::SystemManager<
            $crate::scene::Common<$crate::scene::SceneContext<$tstate, $tcomponents, $name>>
        > for $name
        {
            fn update(&mut self,
                ctx: &mut $crate::scene::Common<$crate::scene::SceneContext<$tstate, $tcomponents, $name>>,
                state: & $tstate)
            {
                macro_rules! update_call {
                    (updates $afsb: ident $afsb1: ty) => {
                        <$afsb1 as regecs::system::Updatable<
                            $crate::scene::Common<$crate::scene::SceneContext<$tstate, $tcomponents, $name>>>>
                        ::update(&mut self.$afsb, ctx, state);
                    };
                    ($afsb: ident $afsb1: ty) => {};
                }
                $(update_call!($($poption)? $pname $ptype);)*
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

        $(
            $(#[$ctxouter])*
            $access type $ctxname = $crate::scene::SceneContext<$tstate, $tcomponents, $name>;
        )?
    };
}

pub use build_component_manager;
pub use build_system_manager;

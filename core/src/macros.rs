pub use impls::impls;
pub use paste::paste;

#[macro_export]
macro_rules! pool_type {
    ($i: ty) => {
        <$i as regecs::component::interface::Component>::Pool
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
            #[derive(Default)]
            struct $name
            {
                $(
                    [<sys_$system:snake>]: $system,
                )*
            }

            $(
                impl SystemProvider<$system> for $name
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

            impl SystemList<$tstate, $tcomponents> for $name
            {
                fn update(&mut self, ctx: SystemContext<$tstate, $tcomponents>)
                {
                    $(
                        if <$system as System<$tstate, $tcomponents>>::UPDATABLE {
                            //self.[<sys_$system:snake>].update(ctx);
                        }
                    )*
                }
            }
        }
    };
}

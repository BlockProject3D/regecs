#[macro_export]
macro_rules! impl_component_manager {
    (
        $name: ty { $(($pname: ident : $ptype: ty))* }
    ) => {
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
    };
}

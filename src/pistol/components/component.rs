pub trait Component {
    fn is_tick(&self) -> bool;
    fn is_tick_mut(&self) -> bool;
}

#[macro_export]
macro_rules! impl_component {
    ($t: ty, $ti: expr, $tim: expr) => {
        impl $crate::pistol::Component for $t {
            #[inline]
            fn is_tick(&self) -> bool {
                $ti
            }

            #[inline]
            fn is_tick_mut(&self) -> bool {
                $tim
            }
        }
    };
}

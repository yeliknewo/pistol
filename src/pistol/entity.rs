use std::sync::{Arc};
use std::hash::{Hash};
use id_alloc::*;
use pistol::components::{Transform, Renderable};
use pistol::world::{World};

pub trait Entity<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send, T: Entity<I, T>> : Send + Sync {
    fn get_id(&self) -> I;
    fn get_renderable(&self) -> Option<&Box<Renderable>>;
    fn get_transform(&self) -> Option<&Box<Transform>>;
    fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>>;
    fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>>;
    fn tick(&self, dt: f64, world: Arc<World<I, T>>);
    fn tick_mut(&mut self, manager: &mut Node<I>, world: &mut World<I, T>, tick_2_layer: I);
    fn get_tick_mut_layers(&self) -> 
    fn is_tick(&self) -> bool;
    fn is_tick_mut(&self) -> bool;
}

#[macro_export]
macro_rules! impl_component_box_with_entity {
    ($t: ty, $p: ident, $c: ty, $so: ident, $s: ident, $w: ident, $g: ident, $m: ident, $ta: ident, $gi: ident) => (
        impl $t {
            #[inline]
            pub fn $so (&mut self, $p: Option<$c>) {
                self.$p = $p;
            }

            #[inline]
            pub fn $s (&mut self, $p: $c) {
                self.$so(Some($p));
            }

            #[inline]
            pub fn $w (mut self, $p: $c) -> $t {
                self.$s($p);
                self
            }

            #[inline]
            pub fn $g (&self) -> Option<&$c> {
                self.$p.as_ref()
            }

            #[inline]
            pub fn $m (&mut self) -> Option<&mut $c> {
                self.$p.as_mut()
            }

            #[inline]
            pub fn $ta (&mut self) -> Option<$c> {
                self.$p.take()
            }

            #[inline]
            pub fn $gi (&mut self, $p: $c) {
                self.$p = Some($p);
            }
        }
    )
}

#[macro_export]
macro_rules! impl_component_with_entity {
    ($t: ty, $p: ident, $c: ty, $so: ident, $s: ident, $w: ident, $g: ident, $m: ident, $ta: ident, $gi: ident) => (
        impl $t {
            #[inline]
            pub fn $so (&mut self, $p: Option<Box<$c>>) {
                self.$p = $p;
            }

            #[inline]
            pub fn $s (&mut self, $p: $c) {
                self.$so(Some(Box::new($p)));
            }

            #[inline]
            pub fn $w (mut self, $p: $c) -> $t {
                self.$s($p);
                self
            }

            #[inline]
            pub fn $g (&self) -> Option<&Box<$c>> {
                self.$p.as_ref()
            }

            #[inline]
            pub fn $m (&mut self) -> Option<&mut Box<$c>> {
                self.$p.as_mut()
            }

            #[inline]
            pub fn $ta (&mut self) -> Option<Box<$c>> {
                self.$p.take()
            }

            #[inline]
            pub fn $gi (&mut self, $p: Box<$c>) {
                self.$p = Some($p);
            }
        }
    )
}

#[macro_export]
macro_rules! impl_entity {
    ($id: ident, $r: ident, $tr: ident) => (
        #[inline]
        fn get_id(&self) -> Id {
            self.$id.clone()
        }

        #[inline]
        fn get_renderable(&self) -> Option<&Box<Renderable>> {
            self.$r.as_ref()
        }

        #[inline]
        fn get_transform(&self) -> Option<&Box<Transform>> {
            self.$tr.as_ref()
        }

        #[inline]
        fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>> {
            self.$r.as_mut()
        }

        #[inline]
        fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>> {
            self.$tr.as_mut()
        }
    )
}

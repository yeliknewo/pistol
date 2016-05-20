extern crate piston_window;
extern crate scoped_threadpool;
extern crate fnv;
extern crate sdl2_window;
extern crate num_cpus;

pub use piston_window::*;
pub use sdl2_window::{Sdl2Window};

mod game;
mod world;
mod entity;
mod id;
mod sync_data;
mod components;

pub use self::game::{Game};
pub use self::entity::{Entity};
pub use self::id::{Id, IdManager};
pub use self::components::{Component, Transform, Renderable, Map2d, Map2dCoords, Map3d, Map3dCoords, Container, Name};
pub use self::world::{World};

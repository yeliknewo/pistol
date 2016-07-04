pub extern crate piston_window;
pub extern crate scoped_threadpool;
pub extern crate fnv;
pub extern crate sdl2_window;
pub extern crate num_cpus;
pub extern crate id_alloc;

pub mod pistol {
    pub mod game;
    pub mod world;
    pub mod entity;
    pub mod sync_data;
    pub mod components;
    pub mod layer;

    pub use self::game::*;
    pub use self::world::*;
    pub use self::entity::*;
    pub use self::sync_data::*;
    pub use self::components::*;
    pub use self::layer::*;
}

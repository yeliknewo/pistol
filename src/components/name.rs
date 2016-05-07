use world::{World};
use id::{Id};
use entity::{Entity};

#[derive(Debug)]
pub struct Name {
    name: &'static str,
}

impl_component!(Name, false, false);

impl Name {
    pub fn new<T: Entity<T>>(name: &'static str, id: Id, world: &mut World<T>) -> Name {
        world.register_name(id, name);
        Name {
            name: name,
        }
    }
}

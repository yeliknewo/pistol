use std::sync::{Arc};
use scoped_threadpool::{Pool};
use piston_window::{PistonWindow, clear, UpdateEvent, BuildFromWindowSettings, Window, AdvancedWindow, OpenGLWindow, GenericEvent};
use num_cpus;

use world::{World};
use entity::{Entity};
use id::{IdManager};

pub struct Game<T: Entity<T>> {
    world: Arc<World<T>>,
    thread_pool: Pool,
}

impl<T: Entity<T>> Game<T> {
    pub fn new() -> Game<T> {
        Game {
            world: Arc::new(World::new()),
            thread_pool: Pool::new(num_cpus::get() as u32),
        }
    }

    #[inline]
    pub fn get_world(&self) -> Arc<World<T>> {
        self.world.clone()
    }

    #[inline]
    pub fn get_mut_world(&mut self) -> &mut World<T> {
        if let Some(world) = Arc::get_mut(&mut self.world) {
            return world;
        } else {
            panic!("Get mut World was None");
        }
    }

    pub fn run<W>(&mut self, manager: &mut IdManager, window: &mut PistonWindow<W>) where W: BuildFromWindowSettings + Window + AdvancedWindow + OpenGLWindow, W::Event: GenericEvent {
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let world = self.get_world();
                for layer in world.get_active_layers() {
                    for entity_id in world.get_render_ids().get(layer).expect("Active layer wasn't a layer").iter() {
                        if let Some(entity) = world.get_entity_by_id(entity_id.clone()) {
                            if let Some(renderable) = entity.get_renderable() {
                                renderable.draw_2d(c, g);
                            }
                        }
                    }
                }
            });
            if let Some(args) = e.update_args() {
                //Immutable Multithreaded Tick
                {
                    let world = self.get_world();
                    let dt = args.dt;
                    self.thread_pool.scoped(|scope| {
                        for entity_id in world.get_tick_ids() {
                            let world = world.clone();
                            scope.execute(move || {
                                if let Some(entity) = world.get_entity_by_id(entity_id.clone()) {
                                    entity.tick(dt, world.clone());
                                }
                            });
                        }
                    });
                }

                //Mutable Single Thread Tick
                {
                    let mut world = self.get_mut_world();
                    if let Some(tick_mut_ids) = world.take_tick_mut_ids() {
                        for id in tick_mut_ids.iter() {
                            if let Some(mut entity) = world.take_entity_by_id(id.clone()) {
                                entity.tick_mut(manager, &mut world);
                                world.give_entity(entity);
                            }
                        }
                        world.give_tick_mut_ids(tick_mut_ids);
                    }
                }
            }
        }
    }
}

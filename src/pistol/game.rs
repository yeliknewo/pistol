use std::sync::{Arc};
use scoped_threadpool::{Pool};
use piston_window::{PistonWindow, clear, UpdateEvent, BuildFromWindowSettings, Window, AdvancedWindow, OpenGLWindow, GenericEvent};
use num_cpus;
use std::hash::{Hash};
use id_alloc::*;

use pistol::world::{World};
use pistol::entity::{Entity};

pub struct Game<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send, T: Entity<I, T>> {
    world: Arc<World<I, T>>,
    thread_pool: Pool,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send, T: Entity<I, T>> Game<I, T> {
    pub fn new() -> Game<I, T> {
        Game {
            world: Arc::new(World::new()),
            thread_pool: Pool::new(num_cpus::get() as u32),
        }
    }

    #[inline]
    pub fn get_world(&self) -> Arc<World<I, T>> {
        self.world.clone()
    }

    #[inline]
    pub fn get_mut_world(&mut self) -> &mut World<I, T> {
        if let Some(world) = Arc::get_mut(&mut self.world) {
            return world;
        } else {
            panic!("Get mut World was None");
        }
    }

    pub fn run<W>(&mut self, manager: &mut Node<I>, window: &mut PistonWindow<W>) where W: BuildFromWindowSettings + Window + AdvancedWindow + OpenGLWindow, W::Event: GenericEvent {
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let layers = self.world.get_render_layers();
                for layer_id in layers.get_active_layers() {
                    if let Some(layer) = layers.get_layer(layer_id) {
                        for entity_id in layer {
                            if let Some(entity) = self.world.get_entity_by_id(*entity_id) {
                                if let Some(renderable) = entity.get_renderable() {
                                    renderable.draw_2d(c, g);
                                }
                            }
                        }
                    }
                }
            });
            if let Some(args) = e.update_args() {
                //Immutable Multithreaded Tick
                {
                    let dt = args.dt;
                    let layers = self.get_mut_world().take_tick_layers().expect("Tick layers was none");
                    let world = self.get_world();
                    self.thread_pool.scoped(|scope| {
                        for layer_id in layers.get_active_layers() {
                            if let Some(layer) = layers.get_layer(layer_id) {
                                for entity_id in layer {
                                    if let Some(entity) = world.get_entity_by_id(*entity_id) {
                                        let world = world.clone();
                                        scope.execute(move || {
                                            entity.tick(dt, world.clone());
                                        });
                                    }
                                }
                            }
                        }
                    });
                    self.get_mut_world().give_tick_layers(layers);

                    // let dt = args.dt;
                    // let layers = self.get_mut_world().take_tick_layers().expect("Layers was not in world");
                    // let world = self.get_world();
                    // self.thread_pool.scoped(|scope| {
                    //     for layer_id in layers.get_active_layers() {
                    //         if let Some(layer_2) = layers.get_active_layers_2().get(layer_id) {
                    //             for layer_id_2 in layer_2 {
                    //                 if let Some(layer) = layers.get_layer(*layer_id, *layer_id_2) {
                    //                     for entity_id in layer {
                    //                         let world = world.clone();
                    //                         scope.execute(move || {
                    //                             if let Some(entity) = world.get_entity_by_id(*entity_id) {
                    //                                 entity.tick(dt, world.clone());
                    //                             }
                    //                         })
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }
                    // });
                    // self.get_mut_world().give_tick_layers(layers);
                }

                //Mutable Single Thread Tick
                {
                    let layers = self.get_mut_world().take_tick_mut_layers().expect("Tick mut layers was none");
                    {
                        let mut world = self.get_mut_world();
                        for layer_id in layers.get_active_layers() {
                            if let Some(layer_2) = layers.get_active_layers_2().get(layer_id) {
                                for layer_id_2 in layer_2 {
                                    if let Some(layer) = layers.get_layer(*layer_id, *layer_id_2) {
                                        for entity_id in layer {
                                            if let Some(mut entity) = world.take_entity_by_id(*entity_id) {
                                                entity.tick_mut(manager, &mut world, *layer_id_2);
                                                world.give_entity(entity);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.get_mut_world().give_tick_mut_layers(layers);
                    // let mut world = self.get_mut_world();


                    // if let Some(tick_mut_ids) = world.take_tick_mut_ids() {
                    //     for id in tick_mut_ids.iter() {
                    //         if let Some(mut entity) = world.take_entity_by_id(id.clone()) {
                    //             entity.tick_mut(manager, &mut world);
                    //             world.give_entity(entity);
                    //         }
                    //     }
                    //     world.give_tick_mut_ids(tick_mut_ids);
                    // }
                }
            }
        }
    }
}

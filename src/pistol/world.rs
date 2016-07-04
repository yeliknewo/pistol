use std::hash::{Hash};
use std::collections::{HashMap};
// use std::collections::hash_state::{DefaultState};
// use fnv::{FnvHasher};

use pistol::entity::{Entity};
use pistol::layer::{Layer1, Layer2};
use id_alloc::*;

pub struct World<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send, T: Entity<I, T>> {
    // entities: HashMap<Id, T, DefaultState<FnvHasher>>,
    entities: HashMap<I, T>,
    names: HashMap<&'static str, I>,
    tick_layers: Option<Layer1<I>>,
    tick_mut_layers: Option<Layer2<I>>,
    render_layers: Layer1<I>,
    // tick_ids: HashSet<I>,
    // tick_mut_ids: Option<HashSet<I>>,
    // render_ids: HashMap<u8, HashSet<I>>,
    // active_layers: Vec<u8>,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send, T: Entity<I, T>> World<I, T> {
    #[inline]
    pub fn new() -> World<I, T> {
        World {
            entities: HashMap::new(),
            names: HashMap::new(),
            tick_layers: Some(Layer1::new()),
            tick_mut_layers: Some(Layer2::new()),
            render_layers: Layer1::new(),
            // tick_ids: HashSet::new(),
            // tick_mut_ids: Some(HashSet::new()),
            // render_ids: HashMap::new(),
            // active_layers: vec!(),
        }
    }

    #[inline]
    pub fn register_name(&mut self, id: I, name: &'static str) {
        if !self.names.contains_key(name) {
            self.names.insert(name, id);
        } else {
            panic!("Name already in use");
        }
    }

    #[inline]
    pub fn deregister_name(&mut self, name: &'static str) {
        self.names.remove(name);
    }

    #[inline]
    pub fn get_entity_by_name(&self, name: &'static str) -> Option<&T> {
        self.get_entity_by_id(self.names.get(name).expect("Name was not found").clone())
    }

    #[inline]
    pub fn get_mut_entity_by_name(&mut self, name: &'static str) -> Option<&mut T> {
        let id = self.names.get(name).expect("Name was not found").clone();
        self.get_mut_entity_by_id(id)
    }

    #[inline]
    pub fn get_entity_by_id(&self, id: I) -> Option<&T> {
        self.entities.get(&id)
    }

    #[inline]
    pub fn get_mut_entity_by_id(&mut self, id: I) -> Option<&mut T> {
        self.entities.get_mut(&id)
    }

    #[inline]
    pub fn add_entity(&mut self, entity: T) {
        let id = entity.get_id();
        self.entities.insert(id.clone(), entity);
        self.add_event_ids(id);
    }

    #[inline]
    pub fn take_entity_by_id(&mut self, id: I) -> Option<T> {
        self.entities.remove(&id)
    }

    #[inline]
    pub fn take_entity_by_name(&mut self, name: &'static str) -> Option<T> {
        self.entities.remove(self.names.get(name).expect("Name was not found"))
    }

    #[inline]
    pub fn give_entity(&mut self, entity: T) {
        self.entities.insert(entity.get_id(), entity);
    }

    #[inline]
    pub fn get_entities(&self) -> &HashMap<I, T> {
        &self.entities
    }

    #[inline]
    pub fn get_mut_entities(&mut self) -> &mut HashMap<I, T> {
        &mut self.entities
    }

    #[inline]
    pub fn get_render_layers(&self) -> &Layer1<I> {
        &self.render_layers
    }

    #[inline]
    pub fn take_tick_layers(&mut self) -> Option<Layer1<I>> {
        self.tick_layers.take()
    }

    pub fn give_tick_layers(&mut self, tick_layers: Layer1<I>) {
        self.tick_layers = Some(tick_layers);
    }

    #[inline]
    pub fn take_tick_mut_layers(&mut self) -> Option<Layer2<I>> {
        self.tick_mut_layers.take()
    }

    #[inline]
    pub fn give_tick_mut_layers(&mut self, tick_mut_layers: Layer2<I>) {
        self.tick_mut_layers = Some(tick_mut_layers);
    }

    fn add_event_ids(&mut self, id: I) {
        let mut tick = false;
        let mut tick_mut = false;
        let mut render = false;
        let mut layer = 0;

        if let Some(entity) = self.get_entity_by_id(id.clone()) {
            tick = entity.is_tick();
            tick_mut = entity.is_tick_mut();
            if let Some(renderable) = entity.get_renderable() {
                render = true;
                layer = renderable.get_layer();
            }
        }

        // if tick {
        //     self.tick_ids.insert(id.clone());
        // }
        //
        // if tick_mut {
        //     if let Some(ref mut tick_mut_ids) = self.tick_mut_ids {
        //         tick_mut_ids.insert(id.clone());
        //     }
        // }
        //
        // if render {
        //     if self.render_ids.contains_key(&layer) {
        //         self.render_ids.get_mut(&layer).expect("Render ids was none somehow").insert(id.clone());
        //     } else {
        //         let mut layer_set = HashSet::new();
        //         layer_set.insert(id.clone());
        //         self.render_ids.insert(layer, layer_set);
        //     }
        //     self.active_layers.push(layer);
        //     self.active_layers.sort();
        //     self.active_layers.dedup();
        // }
    }

    #[inline]
    fn remove_event_ids(&mut self, id: I) {
        // self.tick_ids.remove(&id);
        // if let Some(ref mut tick_mut_ids) = self.tick_mut_ids {
        //     tick_mut_ids.remove(&id);
        // }
        // let mut layer = 0;
        // if let Some(entity) = self.get_entity_by_id(id.clone()) {
        //     if let Some(renderable) = entity.get_renderable() {
        //         layer = renderable.get_layer();
        //     }
        // }
        // if let Some(layer_set) = self.render_ids.get_mut(&layer) {
        //     layer_set.remove(&id);
        // }
    }
}

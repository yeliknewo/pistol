use std::hash::{Hash};
use std::collections::{HashSet, HashMap};
use id_alloc::*;

pub struct Layer1<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send> {
    layers: Option<HashMap<I, HashSet<I>>>,
    active_layers: Vec<I>,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send> Layer1<I> {
    pub fn new() -> Layer1<I> {
        Layer1 {
            layers: Some(HashMap::new()),
            active_layers: vec!(),
        }
    }

    pub fn add(&mut self, layer_id: I, item: I) {
        if let Some(mut layers) = self.layers.take() {
            if layers.contains_key(&layer_id) {
                layers.get_mut(&layer_id).expect("Layer Id was somehow not in layers after being in layers").insert(item);
            } else {
                let mut layer_set = HashSet::new();
                layer_set.insert(item);
                layers.insert(layer_id.clone(), layer_set);
            }
            self.active_layers.push(layer_id);
            self.active_layers.sort();
            self.active_layers.dedup();
            self.layers = Some(layers);
        }
    }

    pub fn remove(&mut self, layer_id: I, item: I) {
        if let Some(layers) = self.layers.as_mut() {
            if let Some(layer_set) = layers.get_mut(&layer_id) {
                layer_set.remove(&item);
            }
        }
    }

    pub fn get_layer(&self, layer_id: &I) -> Option<&HashSet<I>> {
        if let Some(layers) = self.layers.as_ref() {
            layers.get(&layer_id)
        } else {
            None
        }
    }

    pub fn get_all(&self) -> Option<&HashMap<I, HashSet<I>>> {
        self.layers.as_ref()
    }

    pub fn get_active_layers(&self) -> &Vec<I> {
        &self.active_layers
    }

    pub fn take_all(&mut self) -> Option<HashMap<I, HashSet<I>>> {
        self.layers.take()
    }

    pub fn give_all(&mut self, all: HashMap<I, HashSet<I>>) {
        self.layers = Some(all);
    }
}


pub struct Layer2<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send> {
    layers: Option<HashMap<I, HashMap<I, HashSet<I>>>>,
    active_layers_2: HashMap<I, Vec<I>>,
    active_layers: Vec<I>,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash + Sync + Send> Layer2<I> {
    pub fn new() -> Layer2<I> {
        Layer2 {
            layers: Some(HashMap::new()),
            active_layers_2: HashMap::new(),
            active_layers: vec!(),
        }
    }

    pub fn add(&mut self, layer_id: I, layer_2_id: I, item: I) {
        if let Some(mut layers) = self.layers.take() {
            if let Some(mut layers2) = layers.remove(&layer_id) {
                if layers2.contains_key(&layer_2_id) {
                    layers2.get_mut(&layer_id).expect("Layer id was somehow not in layers after being in layers").insert(item);
                } else {
                    let mut layer_set = HashSet::new();
                    layer_set.insert(item);
                    layers2.insert(layer_id, layer_set);
                }
                layers.insert(layer_id, layers2);
            } else {
                let mut layers2 = HashMap::new();
                let mut layer_set = HashSet::new();
                layer_set.insert(item);
                layers2.insert(layer_2_id, layer_set);
                layers.insert(layer_id, layers2);
            }
            if let Some(mut active_layer_2) = self.active_layers_2.remove(&layer_2_id) {
                active_layer_2.push(layer_2_id);
                active_layer_2.sort();
                active_layer_2.dedup();
                self.active_layers_2.insert(layer_2_id, active_layer_2);
            } else {
                let mut active_layer_2 = vec!();
                active_layer_2.push(layer_2_id);
                self.active_layers_2.insert(layer_2_id, active_layer_2);
            }
        }
    }

    #[inline]
    pub fn get_active_layers(&self) -> &Vec<I> {
        &self.active_layers
    }

    #[inline]
    pub fn get_active_layers_2(&self) -> &HashMap<I, Vec<I>> {
        &self.active_layers_2
    }

    #[inline]
    pub fn get_layer(&self, layer_id: I, layer_id_2: I) -> Option<&HashSet<I>> {
        if let Some(layers) = self.layers.as_ref() {
            if let Some(layers2) = layers.get(&layer_id) {
                layers2.get(&layer_id_2)
            } else {
                None
            }
        } else {
            None
        }
    }
}

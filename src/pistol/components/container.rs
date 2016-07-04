use std::hash::{Hash};
use id_alloc::*;

pub struct Container<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash> {
    ids: Vec<I>,
}

// impl_component!(Container, false, false);

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash> Container<I> {
    pub fn new() -> Container<I> {
        Container {
            ids: vec!(),
        }
    }

    pub fn get_ids(&self) -> &Vec<I> {
        &self.ids
    }

    pub fn add_id(&mut self, id: I) {
        self.ids.push(id);
        self.ids.sort();
        self.ids.dedup();
    }

    pub fn remove_id(&mut self, id: I) {
        for i in 0..self.ids.len() {
            if self.ids.get(i) == Some(&id) {
                self.ids.remove(i);
                break;
            }
        }
    }
}

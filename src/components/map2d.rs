use std::collections::{HashMap};
use std::hash::{Hash};

use id::{Id};
use super::{Map2dCoords};

pub struct Map2d<T: Hash + Eq + Copy> {
    tiles: HashMap<Box<Map2dCoords<T>>, Id>,
}

impl<T: Copy + Eq + Hash> super::Component for Map2d<T> {
    fn is_tick(&self) -> bool {
        false
    }

    fn is_tick_mut(&self) -> bool {
        false
    }
}

impl<T: Hash + Eq + Copy> Map2d<T>{
    #[inline]
    pub fn new() -> Map2d<T> {
        Map2d {
            tiles: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, coords: Box<Map2dCoords<T>>, id: Id) {
        self.tiles.insert(coords, id);
    }

    #[inline]
    pub fn insert_split(&mut self, x: T, y: T, id: Id) {
        self.insert(Box::new(Map2dCoords::new(x, y)), id);
    }

    #[inline]
    pub fn get(&self, coords: &Map2dCoords<T>) -> Option<Id> {
        match self.tiles.get(coords) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn get_split(&self, x: T, y: T) -> Option<Id> {
        self.get(&Map2dCoords::new(x,y))
    }
}

use std::collections::{HashMap};
use std::hash::{Hash};

use id::{Id};
use super::{Map3dCoords};

pub struct Map3d<T: Hash + Eq + Copy> {
    tiles: HashMap<Box<Map3dCoords<T>>, Id>,
}

impl<T: Copy + Eq + Hash> super::Component for Map3d<T> {
    fn is_tick(&self) -> bool {
        false
    }

    fn is_tick_mut(&self) -> bool {
        false
    }
}

impl<T: Hash + Eq + Copy> Map3d<T>{
    #[inline]
    pub fn new() -> Map3d<T> {
        Map3d {
            tiles: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, coords: Box<Map3dCoords<T>>, id: Id) {
        self.tiles.insert(coords, id);
    }

    #[inline]
    pub fn insert_split(&mut self, x: T, y: T, z: T, id: Id) {
        self.insert(Box::new(Map3dCoords::new(x,y,z)), id);
    }

    #[inline]
    pub fn get(&self, coords: Map3dCoords<T>) -> Option<Id> {
        match self.tiles.get(&coords) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn get_split(&self, x: T, y: T, z: T) -> Option<Id> {
        self.get(Map3dCoords::new(x,y,z))
    }
}

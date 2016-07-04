use std::collections::{HashMap};
use std::hash::{Hash};
use id_alloc::*;

use super::{Map3dCoords};

pub struct Map3d<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Hash + Eq + Copy> {
    tiles: HashMap<Box<Map3dCoords<T>>, I>,
}

// impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Copy + Eq + Hash> super::Component for Map3d<T> {
//     fn is_tick(&self) -> bool {
//         false
//     }
//
//     fn is_tick_mut(&self) -> bool {
//         false
//     }
// }

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Hash + Eq + Copy> Map3d<I, T>{
    #[inline]
    pub fn new() -> Map3d<I, T> {
        Map3d {
            tiles: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, coords: Box<Map3dCoords<T>>, id: I) {
        self.tiles.insert(coords, id);
    }

    #[inline]
    pub fn insert_split(&mut self, x: T, y: T, z: T, id: I) {
        self.insert(Box::new(Map3dCoords::new(x,y,z)), id);
    }

    #[inline]
    pub fn get(&self, coords: Map3dCoords<T>) -> Option<I> {
        match self.tiles.get(&coords) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn get_split(&self, x: T, y: T, z: T) -> Option<I> {
        self.get(Map3dCoords::new(x,y,z))
    }
}

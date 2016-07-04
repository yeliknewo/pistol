use std::collections::{HashMap};
use std::hash::{Hash};
use id_alloc::*;

use super::{Map2dCoords};

pub struct Map2d<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Hash + Eq + Copy> {
    tiles: HashMap<Box<Map2dCoords<T>>, I>,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Copy + Eq + Hash> super::Component for Map2d<I, T> {
    fn is_tick(&self) -> bool {
        false
    }

    fn is_tick_mut(&self) -> bool {
        false
    }
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Hash + Eq + Copy> Map2d<I, T>{
    #[inline]
    pub fn new() -> Map2d<I, T> {
        Map2d {
            tiles: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, coords: Box<Map2dCoords<T>>, id: I) {
        self.tiles.insert(coords, id);
    }

    #[inline]
    pub fn insert_split(&mut self, x: T, y: T, id: I) {
        self.insert(Box::new(Map2dCoords::new(x, y)), id);
    }

    #[inline]
    pub fn get(&self, coords: &Map2dCoords<T>) -> Option<I> {
        match self.tiles.get(coords) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn get_split(&self, x: T, y: T) -> Option<I> {
        self.get(&Map2dCoords::new(x,y))
    }
}

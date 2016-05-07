use std::hash::{Hash};

#[derive(Hash, Eq, Copy, Clone, PartialEq)]
pub struct Map3dCoords<T: Hash + Eq + Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy + Eq + Hash> super::Component for Map3dCoords<T> {
    fn is_tick(&self) -> bool {
        false
    }

    fn is_tick_mut(&self) -> bool {
        false
    }
}

impl<T: Hash + Eq + Copy> Map3dCoords<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Map3dCoords<T> {
        Map3dCoords {
            x: x,
            y: y,
            z: z,
        }
    }

    #[inline]
    pub fn get_x(&self) -> T {
        self.x
    }

    #[inline]
    pub fn get_y(&self) -> T {
        self.y
    }

    #[inline]
    pub fn get_z(&self) -> T {
        self.z
    }

    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    #[inline]
    pub fn set_z(&mut self, z: T) {
        self.z = z;
    }
}

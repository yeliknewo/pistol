#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Id {
    id: IdSize,
}

impl Id {
    pub fn new(manager: &mut IdManager) -> Id {
        manager.next()
    }
}

type IdSize = u32;

pub struct IdManager {
    buffer_size: usize,
    unused_ids_start: IdSize,
    unused_ids_buffer: Vec<Id>,
}

impl IdManager {
    pub fn new_default() -> IdManager {
        IdManager::new(128)
    }

    pub fn new(buffer_size: usize) -> IdManager {
        IdManager {
            buffer_size: buffer_size,
            unused_ids_start: 0,
            unused_ids_buffer: vec!(),
        }
    }

    fn next(&mut self) -> Id {
        if let Some(id) = self.unused_ids_buffer.pop() {
            return id;
        }
        while self.unused_ids_buffer.len() < self.buffer_size {
            self.unused_ids_buffer.push(Id {
                id: self.unused_ids_start,
            });
            if let Some(next_id) = self.unused_ids_start.checked_add(1) {
                self.unused_ids_start = next_id;
            } else {
                panic!("Ran out of Ids");
            }
        }
        return self.unused_ids_buffer.pop().expect("Error in Next in IdManager");
    }
}

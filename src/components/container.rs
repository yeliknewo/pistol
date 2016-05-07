use id::{Id};

pub struct Container {
    ids: Vec<Id>,
}

impl_component!(Container, false, false);

impl Container {
    pub fn new() -> Container {
        Container {
            ids: vec!(),
        }
    }

    pub fn get_ids(&self) -> &Vec<Id> {
        &self.ids
    }

    pub fn add_id(&mut self, id: Id) {
        self.ids.push(id);
        self.ids.sort();
        self.ids.dedup();
    }

    pub fn remove_id(&mut self, id: Id) {
        for i in 0..self.ids.len() {
            if self.ids.get(i) == Some(&id) {
                self.ids.remove(i);
                break;
            }
        }
    }
}

use super::library::{Entry, Library};

pub struct Selector {
    lib: Library
}

impl Selector {
    pub fn new(lib: Library) -> Self {
        Selector {
            lib
        }
    }

    pub fn select(self) -> Entry {
        self.lib.entries[0].clone()
    }
}

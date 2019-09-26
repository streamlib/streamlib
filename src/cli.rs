use super::library::Library;

pub struct Selector {
    lib: Library
}

impl Selector {
    pub fn new(lib: Library) -> Self {
        Selector {
            lib
        }
    }

    pub fn select(self) -> String {
        self.lib.entries[0].url.clone()
    }
}

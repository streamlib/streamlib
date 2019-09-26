
pub struct Selector {

}

impl Selector {
    pub fn new() -> Self {
        Selector {
        }
    }

    pub fn select(self) -> String {
        String::from("http://somafm.com/groovesalad.pls")
    }
}

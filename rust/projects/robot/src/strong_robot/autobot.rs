#[derive(Debug)]
pub struct Robot {
    name: String,
}
impl Robot {
    pub fn new(name: String) -> Self {
        Robot { name }
    }
}

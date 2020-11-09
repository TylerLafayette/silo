pub struct Service {}

impl Service {
    pub fn new() -> Service {
        Service {}
    }

    pub fn say_hello(&self) {
        println!("Hello!");
    }
}

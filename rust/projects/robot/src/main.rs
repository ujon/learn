mod decepticon;
mod strong_robot;

use decepticon::Robot;

mod robot {
    #[derive(Debug)]
    pub struct Robot {
        name: String,
    }
    impl Robot {
        pub fn new(name: String) -> Self {
            Robot { name }
        }
    }
}
fn main() {
    let robot = robot::Robot::new(String::from("T1"));
    println!("(1) {:#?}", robot);

    let robot = decepticon::Robot::new(String::from("T2"));
    println!("(2) {:#?}", robot);

    let robot = strong_robot::autobot::Robot::new(String::from("T2"));
    println!("(3) {:#?}", robot);
}

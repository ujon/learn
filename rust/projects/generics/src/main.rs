use num_traits::{Float, FromPrimitive, ToPrimitive};
fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn solve2<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve3<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}
fn main() {
    println!("(1) {}", solve(3.0, 4.0));
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    let a_f64 = a.to_f64().unwrap();
    println!("(2) {}", solve(a_f64, b));
    println!("(3) {}", solve2(a, b));
    println!("(4) {}", solve3(a, 4));
}

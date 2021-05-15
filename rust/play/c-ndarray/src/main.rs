use ndarray::Array;
use std::f64::consts::PI;

fn main() {
    let space = Array::linspace(0., 2f64 * PI, 32);
    let sin_space = space.map(|x| x.sin().abs());
    println!("{:?}", space);
    println!("{:?}", sin_space);
}

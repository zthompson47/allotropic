use dwt::{transform, wavelet::Haar, Operation::Forward};
fn main() {
    let mut data = vec![0., 1., 0., 1., 0., 1., 0., 1.3e3];
    println!("BEFORE: {:?}", data);
    let haar = Haar::new();
    let wavelet = transform(&mut data, Forward, &haar, 3);
    println!("AFTER: {:?}", data);
}

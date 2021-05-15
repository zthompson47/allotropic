use ndarray::prelude::*;

fn main() {
    let m = mean::<f32>(vec![1.2, 43., 234.].as_slice());
    println!("{}", m.unwrap());
}

fn mean<T>(data: &[T]) -> Option<T>
where
    T: cpal::Sample
        + std::ops::Div<Output = T>
        + num_traits::identities::Zero
        + num_traits::cast::FromPrimitive,
{
    let a = Array1::from_vec(data.to_vec());

    a.mean()
}

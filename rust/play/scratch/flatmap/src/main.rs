// https://users.rust-lang.org/t/vector-to-image-grayscale/60408/7
fn main() {
    let width = 1058;
    let height = 323;
    let mut mat = vec![vec![0u8; width as usize]; height as usize];
    let redacted_demo_vec = vec![1u8; width * height];

    let _a = &mut mat[..height];
    let _b = mat.iter_mut();

    mat[..height]
        .iter_mut()
        //.flat_map(|s| &mut s[..width])
        .flat_map(|s| s.iter_mut())
        .zip(&redacted_demo_vec)
        .for_each(|(mat, demo)| *mat = *demo);

    println!("{:?}", mat.len());

    let mat: Vec<_> = redacted_demo_vec
        .chunks_exact(width)
        .take(height)
        .map(|s| s.to_vec())
        .collect();

    println!("{:?}", mat.len());

    let words = ["alpha", "beta", "gamma"];
    let merged: String = words.iter()
        .map(|s| s.chars())
        .flatten()
        .collect();

    println!("{:?}", merged);

    let merged: String = words.iter()
        .flat_map(|s| s.chars())
        .collect();

    println!("{:?}", merged);
}

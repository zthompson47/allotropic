use std::borrow::Cow;

fn main() {
    let mut a = mkstring();
    println!("{:p}", a.to_mut());

    let arr = vec![1, 2, -3, 4, -5];
    let mut c = Cow::from(arr);
    abs_all(&mut c);
    println!("{:?}", c);

    let mut arr = vec![1, 2, -3, 4, -5];
    abs_all2(&mut arr);
    println!("{:?}", arr);
}

fn mkstring<'a>() -> Cow<'a, str> {
    let s = "hello".to_string();
    let mut c = Cow::from(s);
    println!("{:p}", c.to_mut());
    c
}

/*
fn mkstring2<'a>() -> &'a str {
    let s = "hello".to_string();
    s.as_str()
}
*/

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
}

fn abs_all2(input: &mut [i32]) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input[i] = -v;
        }
    }
}

#[cfg(test)]
mod tests {
}

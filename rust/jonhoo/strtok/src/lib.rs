pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimeter: char) -> &'b str {
    if let Some(i) = s.find(delimeter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimeter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_it_works() {
        //let mut x: &'static str = "hello world";
        let mut x = "hello world";
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }

    #[test]
    fn it_works() {
        fn check_is_static(_: &'static str) {}

        let mut x = "jello swirled";
        check_is_static(x);

        let jello = strtok(&mut x, ' ');

        assert_eq!(jello, "jello");
        assert_eq!(x, "swirled");
    }

    #[test]
    fn mut_ref() {
        fn assign<'a>(s: &mut &'a str, x: &'a str) {
            *s = x;
        }
        let x = String::from("asdf");
        let mut xx: &str = &x;
        let z = String::new();
        assign(&mut xx, &z);
        //drop(z);
        println!("{}", x);
    }

    #[test]
    fn bar() {
        let mut y = true;
        let mut z = &mut y;

        let x = Box::new(true);
        let x: &'static mut bool = Box::leak(x);

        // ignore - gets rid of compiler warnings
        let _ = z;

        z = x;

        // ignore - gets rid of compiler warnings
        let _ = z;
    }

    /*
    #[test]
    #[allow(clippy::drop_ref)]
    fn scratch() {
        let mut a: &'static str = "asdf";
        let b: &'static str = a;
        let cc = String::new();
        let c: &str = &cc;
        a = c;
        drop(a);
        drop(b);
    }
    */
}

fn main() {
    let vector = vec![1, 3, 5, 7, 9];
    let mut iterator = MyIterator { slice: &vector };
    //for i in iterator {
    //    println!("{:?}", i);
    //}

    let a = iterator.next();
    let b = iterator.next();
    drain(iterator);
}

fn drain<I: Iterator<Item = T>, T: std::fmt::Display>(iter: I) {
    for a in iter {
        println!("{}", a);
    }
}

struct MyIterator<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        //let (element, rest) = self.slice.split_first()?;
        //self.slice = rest;
        //Some(element)

        let element = self.slice.get(0)?;
        self.slice = &self.slice[1..];
        Some(element)

        // Panics - index out of range
        //let element = self.slice.get(0);
        //self.slice = &self.slice[1..];
        //element
    }
}

struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl <'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        // Won't compile dut to lifetime issues..
        //let element = self.slice.get_mut(0)?;
        //self.slice = &mut self.slice[1..];
        //Some(element)

        let slice = &mut self.slice;
        let slice = std::mem::replace(slice, &mut []);
        let (element, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyMutableIterator {
            slice: &mut collection,
        };

        for elem in wrapper {
            *elem += 1;
        }

        assert_eq!(collection.get(0), Some(&2));
    }
}

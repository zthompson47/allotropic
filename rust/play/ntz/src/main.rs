use ntz::si::*;
use ntz::angle::*;

fn main() {
    let inch = inches(22.);
    let gram = grams(99.);
    let meter = meters(4.);
    let kilogram = kilograms(47.);

    if inch > meter {
        println!("{} greater than {}", inch.to_meters(), meter);
    } else {
        println!("{} less than {}", inch.to_meters(), meter);
    }

    let a_to_m = inches(42.);
    println!("a_to_m: {:?}", a_to_m.to_meters());

    //let iii = 47. * i;

    println!("{:?} + {:?} = {:?}", inch, inch, inch + inch);
    println!("{:?} * {:?} = {:?}", inch, gram, inch * gram);
    println!("{:?} * {:?} = {:?}", inch, inch, inch * inch);
    println!("{:?} / {:?} = {:?}", inch, gram, inch / gram);
    println!(
        "{:?}",
        ((inch * gram) + (inch * gram) + (meter * kilogram)) * meter * kilogram
    );
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use ntz::prelude::*;

    use super::*;

    fn eq(x: f64, y: f64) -> bool {
        println!("{} vs {}", x, y);
        (x - y).abs() < f64::EPSILON
    }

    #[test]
    fn homogeneity() {
        let a = grams(99.) * meters(22.) + grams(2323.) * meters(98.);
        //let b = grams(99.) * meters(22.) + grams(2323.) * degrees(98.);
    }

    #[test]
    fn deref() {
        let val = grams(99.);
        //assert!(eq(*val, 99.));
        //assert!(eq(*val.sqrt(), 9.9498743710662));
    }

    #[test]
    fn partial_ord() {
        let inch = inches(22.);
        let meter = meters(4.);
        //assert!(*inch > *meter);
        assert!(inch < meter);
    }

    #[test]
    fn conversion() {
        let i = inches(42.);
        let m = i.to_meters();
        //assert!(eq(*m, 1.0668));
    }

    #[test]
    fn add() {
        let inch = inches(22.);
        let meter = meters(4.);
        assert_float_eq!(inches(66.), inch + inch + inch, abs <= inches(1.));
        assert_float_eq!(millimeters(4558.8), inch + meter, abs <= millimeters(0.1));
    }

    #[test]
    fn sub() {
        let inch = inches(22.);
        let meter = meters(4.);
        let nautical = nautical_miles(1.47);
        println!("--->>>{}", (nautical).norm());
        println!("--->>>{}", (inch).norm());
        println!("--->>>{}", (-inch).norm());
        println!("--->>>{}", (nautical - inch).norm());
        println!("--->>>{}", (nautical - inch - meter).norm());
        assert!(eq(
            (inch - inches(1.) - inches(1.) - inches(1.)).norm(),
            inches(19.).norm()
        ));
        //assert_float_eq!(miles(22.3), nautical - meter - inch, abs <= miles(0.1));
        //assert_eq!(miles(22.3), nautical - meter - inch);
    }

    #[test]
    fn sqrt() {
        let m = meters(42.);
        let g = kilograms(47.);
        let n = g / m.sqrt();
        println!("{:?}", m);
        println!("{:?}", n);
        assert_eq!(m.unit.power(), Some(1.));
        assert_eq!(n.unit.power(), None);
    }

    #[test]
    fn it_works() {
        let inch = inches(22.);
        let gram = grams(99.);
        let meter = meters(4.);
        let kilogram = kilograms(47.);

        if inch > meter {
            println!("{} greater than {}", inch.to_meters(), meter);
        } else {
            println!("{} less than {}", inch.to_meters(), meter);
        }

        let a_to_m = inches(42.);
        println!("a_to_m: {:?}", a_to_m.to_meters());

        //let iii = 47. * i;

        println!("{:?} + {:?} = {:?}", inch, inch, inch + inch);
        println!("{:?} * {:?} = {:?}", inch, gram, inch * gram);
        println!("{:?} * {:?} = {:?}", inch, inch, inch * inch);
        println!("{:?} / {:?} = {:?}", inch, gram, inch / gram);
        println!(
            "{:?}",
            ((inch * gram) + (inch * gram) + (meter * kilogram)) * meter * kilogram
        );
    }
}

use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    fmt::{Display, Formatter, Result},
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};

use paste::paste;

fn main() {
    let inch = inches(22.);
    let gram = grams(99.);
    let meter = meters(4.);
    let kilogram = kilograms(47.);

    if inch > meter {
        println!("{} greater than {}", inch.as_meters(), meter);
    } else {
        println!("{} less than {}", inch.as_meters(), meter);
    }

    let a_to_m = inches(42.);
    println!("a_to_m: {:?}", a_to_m.as_meters());

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

#[derive(Clone, Copy, Debug)]
struct Value<U: Unit> {
    value: f64,
    unit: U,
}

impl<U: Unit + Clone> Value<U> {
    fn sqrt(mut self) -> Self {
        Value {
            value: self.value.sqrt(),
            unit: self.unit.map_power(&|p| p / 2.),
        }
    }
}

impl<U: Unit> Value<U> {
    fn norm(&self) -> f64 {
        self.value * self.unit.factor()
    }
}

impl<U: Unit> Display for Value<U> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "{}", self.value)
    }
}

impl<U: Unit> Deref for Value<U> {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.value
    }
}

impl<U: Unit> PartialEq for Value<U> {
    fn eq(&self, other: &Self) -> bool {
        self.norm() == other.norm()
    }
}

impl<U: Unit> PartialOrd for Value<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.norm().partial_cmp(&other.norm())
    }
}

impl Value<LengthUnit> {
    fn as_meters(&self) -> Value<LengthUnit> {
        Value {
            value: self.norm() / LengthUnit::Meter(1.).factor(),
            unit: LengthUnit::Meter(1.),
        }
    }
}

trait Unit {
    fn factor(&self) -> f64;
    fn power(&self) -> Option<f64>;
    fn map_power<F>(&mut self, f: &F) -> Self
    where
        F: Fn(f64) -> f64;
}

unit! {
    LengthUnit {
        AstronomicalUnit => (astronomical_units, "AE", 1.495979e11),
        CentImeter => (centimeters, "cm", 0.01),
        Decimeter => (decimeters, "dm", 0.1),
        Foot => (feet, "ft", 12. * 0.0254),
        Inch => (inches, "in", 0.0254),
        Kilometer => (kilometers, "km", 1000.),
        LightYear => (light_years, "lj", 9.4607304725808e15),
        Meter => (meters, "m", 1.),
        Mile => (miles, "mi", 1609.344),
        Millimeter => (millimeters, "mm", 0.001),
        NauticalMile => (nautical_miles, "sm", 1852.),
        Parsec => (parsecs, "pc", 3.085678e16),
        Yard => (yards, "yd", 0.9144)
    }
}

unit! {
    MassUnit {
        Gram => (grams, "g", 0.001),
        Kilogram => (kilograms, "kg", 1.)
    }
}

#[derive(Clone, Copy, Debug)]
struct MulUnit<UnitL: Unit, UnitR: Unit>(UnitL, UnitR);

impl<UnitL: Unit, UnitR: Unit> Unit for MulUnit<UnitL, UnitR> {
    fn factor(&self) -> f64 {
        self.0.factor() * self.1.factor()
    }

    fn power(&self) -> Option<f64> {
        None //self.0.power() + self.1.power()
    }

    fn map_power<F>(&mut self, f: &F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        MulUnit(self.0.map_power(&f), self.1.map_power(&f))
    }
}

impl<U: Unit> Add<Value<U>> for Value<U> {
    type Output = Value<U>;

    fn add(self, rhs: Value<U>) -> Value<U> {
        let rval = rhs.norm() / self.unit.factor();
        Value {
            value: self.value + rval,
            unit: self.unit,
        }
    }
}

impl<U: Unit> Sub<Value<U>> for Value<U> {
    type Output = Value<U>;

    fn sub(self, rhs: Value<U>) -> Value<U> {
        let rval = rhs.norm() / self.unit.factor();
        Value {
            value: self.value - rval,
            unit: self.unit,
        }
    }
}

impl<UnitL: Unit, UnitR: Unit> Mul<Value<UnitR>> for Value<UnitL> {
    type Output = Value<MulUnit<UnitL, UnitR>>;

    fn mul(self, rhs: Value<UnitR>) -> Value<MulUnit<UnitL, UnitR>> {
        Value {
            value: self.value * rhs.value,
            unit: MulUnit(self.unit, rhs.unit),
        }
    }
}

impl<UnitL: Unit, UnitR: Unit> Div<Value<UnitR>> for Value<UnitL> {
    type Output = Value<MulUnit<UnitL, UnitR>>;

    fn div(self, mut rhs: Value<UnitR>) -> Value<MulUnit<UnitL, UnitR>> {
        Value {
            value: self.value / rhs.value,
            unit: MulUnit(self.unit, rhs.unit.map_power(&|p| -p)),
        }
    }
}

impl<U: Unit> Neg for Value<U> {
    type Output = Self;

    fn neg(self) -> Self {
        Value {
            value: -self.value,
            unit: self.unit,
        }
    }
}

#[macro_export]
macro_rules! unit {
    ( $unit:ty { $( $var:ident => ($new:ident, $abbr:literal, $factor:expr) ),+ } ) => {
        paste! {
            #[derive(Clone, Copy, Debug)]
            enum $unit {
                $( $var(f64), )+
            }

            impl Unit for $unit {
                fn factor(&self) -> f64 {
                    match self {
                        $( $unit::$var(_) => $factor, )+
                    }
                }

                fn power(&self) -> Option<f64> {
                    match self {
                        $( $unit::$var(p) => Some(*p), )+
                    }
                }

                fn map_power<F>(&mut self, f: &F) -> $unit
                where
                    F: Fn(f64) -> f64,
                {
                    match self {
                        $(
                            $unit::$var(p) => {
                                *p = f(*p);
                                *self
                            },
                        )+
                    }
                }
            }

            $(
                #[allow(dead_code)]
                fn $new(value: f64) -> Value<$unit> {
                    Value {
                        value,
                        unit: $unit::$var(1.),
                    }
                }
            )+
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq(x: f64, y: f64) -> bool {
        println!("{} vs {}", x, y);
        (x - y).abs() < f64::EPSILON
    }

    #[test]
    fn deref() {
        let val = grams(99.);
        assert!(eq(*val, 99.));
        assert!(eq(*val.sqrt(), 9.9498743710662));
    }

    #[test]
    fn partial_ord() {
        let inch = inches(22.);
        let meter = meters(4.);
        assert!(*inch > *meter);
        assert!(inch < meter);
    }

    #[test]
    fn conversion() {
        let i = inches(42.);
        let m = i.as_meters();
        assert!(eq(*m, 1.0668));
    }

    #[test]
    fn add() {
        let inch = inches(22.);
        let meter = meters(4.);
        assert_eq!(inches(66.), inch + inch + inch);
        assert_eq!(millimeters(660.4), inch + meter);
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
        assert!(eq(miles(22.3).norm(), (nautical - meter - inch).norm()));
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
            println!("{} greater than {}", inch.as_meters(), meter);
        } else {
            println!("{} less than {}", inch.as_meters(), meter);
        }

        let a_to_m = inches(42.);
        println!("a_to_m: {:?}", a_to_m.as_meters());

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

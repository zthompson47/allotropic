use std::{cmp::{Ordering, PartialEq, PartialOrd}, fmt::{Display, Formatter, Result}, ops::{Add, Deref, Div, Mul, Neg, Sub}};

use paste::paste;

fn main() {
}

#[derive(Clone, Copy, Debug)]
struct Value<U> {
    value: f64,
    unit: U,
}

impl<U: Unit> Value<U> {
    fn norm(&self) -> f64 {
        println!("]]]]]]]]]]]]]{}/{}[[[[[[[[[[[[[[[[[[[[[", self.value, self.unit.factor());
        self.value * self.unit.factor()
    }
}

impl<U> Display for Value<U> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "{}", self.value)
    }
}

impl<U> Deref for Value<U> {
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
            value: self.norm() / LengthUnit::Meter.factor(),
            unit: LengthUnit::Meter,
        }
    }
}

trait Unit {
    fn factor(&self) -> f64;
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

/*
#[derive(Clone, Copy, Debug)]
enum LengthUnit {
    Inch,
    Meter,
}
*/

/*
impl Unit for LengthUnit {
    fn factor(&self) -> f64 {
        match self {
            LengthUnit::Inch => 0.0254,
            LengthUnit::Meter => 1.,
        }
    }
}
*/

/*
#[derive(Clone, Copy, Debug)]
enum MassUnit {
    Kilogram,
    Gram,
}
*/

#[derive(Clone, Copy, Debug)]
struct MulUnit<UnitL, UnitR>(UnitL, UnitR);

#[derive(Clone, Copy, Debug)]
struct DivUnit<UnitL, UnitR>(UnitL, UnitR);

/*
fn inches(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Inch,
    }
}

fn meters(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Meter,
    }
}

fn grams(value: f64) -> Value<MassUnit> {
    Value {
        value,
        unit: MassUnit::Gram,
    }
}

fn kilograms(value: f64) -> Value<MassUnit> {
    Value {
        value,
        unit: MassUnit::Kilogram,
    }
}
*/

impl<U> Add<Value<U>> for Value<U> {
    type Output = Value<U>;

    fn add(self, rhs: Value<U>) -> Value<U> {
        Value {
            value: self.value + rhs.value,
            unit: self.unit,
        }
    }
}

impl<U> Sub<Value<U>> for Value<U> {
    type Output = Value<U>;

    fn sub(self, rhs: Value<U>) -> Value<U> {
        println!(">>>>>>>{}/{}<<{}<<<<<<<", self.value, rhs.value, self.value - rhs.value);
        Value {
            value: self.value - rhs.value,
            unit: self.unit,
        }
    }
}

impl<UnitL, UnitR> Mul<Value<UnitR>> for Value<UnitL> {
    type Output = Value<MulUnit<UnitL, UnitR>>;

    fn mul(self, rhs: Value<UnitR>) -> Value<MulUnit<UnitL, UnitR>> {
        Value {
            value: self.value * rhs.value,
            unit: MulUnit(self.unit, rhs.unit),
        }
    }
}

impl<UnitL, UnitR> Div<Value<UnitR>> for Value<UnitL> {
    type Output = Value<MulUnit<UnitL, UnitR>>;

    fn div(self, rhs: Value<UnitR>) -> Value<MulUnit<UnitL, UnitR>> {
        Value {
            value: self.value / rhs.value,
            unit: MulUnit(self.unit, rhs.unit),
        }
    }
}

impl <U> Neg for Value<U> {
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
    ( $t:ty { $( $x:ident => ($y:ident, $c:literal, $f:expr) ),+ } ) => {
        paste! {
            #[derive(Clone, Copy, Debug)]
            enum $t {
                $( $x, )+
            }

            impl Unit for $t {
                fn factor(&self) -> f64 {
                    match self {
                        $( $t::$x => $f, )+
                    }
                }
            }

            $(
                #[allow(dead_code)]
                fn $y(value: f64) -> Value<$t> {
                    Value {
                        value,
                        unit: $t::$x,
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
        assert!(eq(val.sqrt(), 9.9498743710662));
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
        assert!(eq((inch - inches(1.) - inches(1.) - inches(1.)).norm(), inches(19.).norm()));
        assert!(eq(miles(22.3).norm(), (nautical - meter - inch).norm()));
        //assert_eq!(miles(22.3), nautical - meter - inch);
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

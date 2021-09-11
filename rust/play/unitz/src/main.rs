use std::fmt::{Debug, Result};

use paste::paste;

fn main() {
    let i = Length::inches(42.);
    let m = Length::meters(2.);
    println!("i: {:?}, m: {:?}", &i, &m);

    let g = Mass::grams(47.);
    let k = Mass::kilograms(1.3);
    println!("g: {:?}, k: {:?}", &g, &k);

    let im = &i * &m;
    println!("{:?}", &im);

    let ig = &i * &g;
    println!("{:?}", &ig);

    let ii = &i * &i;
    println!("{:?}", &ii);
}

trait Value {
    fn value(&self) -> f64;
    fn unit(&self) -> Box<dyn Unit>;
}

impl Debug for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "[V v:{:?}, u:{:?}]",
            self.value(),
            self.unit()
        ))
    }
}

#[derive(Clone, Debug)]
struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Value for Length {
    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Box<dyn Unit> {
        Box::new(self.unit)
    }
}

impl Length {
    impl_units!(
        Length, {
            au => LengthUnit::Au,
            feet => LengthUnit::Foot,
            inches => LengthUnit::Inch,
            kilometers => LengthUnit::Kilometer,
            light_years => LengthUnit::LightYear,
            meters => LengthUnit::Meter,
            miles => LengthUnit::Mile,
            millimeters => LengthUnit::Millimeter,
            parsecs => LengthUnit::Parsec
        }
    );
}
/*
    fn inches(value: f64) -> Box<dyn Value> {
        Box::new(Length {
            value,
            unit: LengthUnit::Inch,
        })
    }

    fn meters(value: f64) -> Box<dyn Value> {
        Box::new(Length {
            value,
            unit: LengthUnit::Meter,
        })
    }
}
*/

#[derive(Clone, Debug)]
struct Mass {
    value: f64,
    unit: MassUnit,
}

impl Value for Mass {
    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Box<dyn Unit> {
        Box::new(self.unit)
    }
}

impl Mass {
    fn kilograms(value: f64) -> Box<dyn Value> {
        Box::new(Mass {
            value,
            unit: MassUnit::Kilogram,
        })
    }

    fn grams(value: f64) -> Box<dyn Value> {
        Box::new(Mass {
            value,
            unit: MassUnit::Gram,
        })
    }
}

struct Derived {
    value: f64,
    unit: DerivedUnit,
}

impl Value for Derived {
    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Box<dyn Unit> {
        Box::new(DerivedUnit {
            lhs: self.unit.lhs.clone(),
            rhs: self.unit.rhs.clone(),
        })
    }
}

impl std::ops::Mul for &Box<dyn Value> {
    type Output = Box<dyn Value>;

    fn mul(self, rhs: Self) -> Box<dyn Value> {
        Box::new(Derived {
            value: self.value() * rhs.value(),
            unit: DerivedUnit {
                lhs: DerivedPart::from(self.unit()),
                rhs: DerivedPart::from(rhs.unit()),
            },
        })
    }
}

trait Unit {
    fn map(&self) -> (f64, String);

    fn factor(&self) -> f64 {
        self.map().0
    }

    fn label(&self) -> String {
        self.map().1
    }
}

impl Debug for dyn Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "[U f:{:?}, n:{:?}]",
            self.factor(),
            self.label()
        ))
    }
}

#[derive(Clone, Copy, Debug)]
enum LengthUnit {
    Au,
    Foot,
    Inch,
    Kilometer,
    LightYear,
    Meter,
    Mile,
    Millimeter,
    Parsec,
}

#[derive(Clone, Copy, Debug)]
enum MassUnit {
    Kilogram,
    Gram,
}

struct DerivedUnit {
    lhs: DerivedPart,
    rhs: DerivedPart,
}

#[derive(Clone)]
struct DerivedPart {
    factor: f64,
    label: String,
}

impl Unit for DerivedPart {
    fn map(&self) -> (f64, String) {
        (self.factor, self.label.clone())
    }
}

impl From<Box<dyn Unit>> for DerivedPart {
    fn from(unit: Box<dyn Unit>) -> Self {
        DerivedPart  {
            factor: unit.factor(),
            label: unit.label(),
        }
    }
}

impl Debug for DerivedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "[f:{:?}, n:{:?}]",
            self.factor(),
            self.label()
        ))
    }
}

impl Unit for DerivedUnit {
    fn map(&self) -> (f64, String) {
        (
            self.lhs.factor() * self.rhs.factor(),
            format!("({} * {})", self.lhs.label(), self.rhs.label()),
        )
    }
}

impl Unit for LengthUnit {
    fn map(&self) -> (f64, String) {
        match self {
            LengthUnit::Au => (1.495979e11, String::from("au")),
            LengthUnit::Foot => (12. * 0.0254, String::from("ft")),
            LengthUnit::Inch => (0.0254, String::from("in")),
            LengthUnit::Kilometer => (1000., String::from("km")),
            LengthUnit::LightYear => (9.4607304725808e15, String::from("ly")),
            LengthUnit::Meter => (1., String::from("m")),
            LengthUnit::Mile => (5280. * 12. * 0.0254, String::from("mi")),
            LengthUnit::Millimeter => (0.001, String::from("mm")),
            LengthUnit::Parsec => (3.085678e16, String::from("ps")),
        }
    }
}

impl Unit for MassUnit {
    fn map(&self) -> (f64, String) {
        match self {
            MassUnit::Kilogram => (1., String::from("kg")),
            MassUnit::Gram => (0.001, String::from("g")),
        }
    }
}

#[macro_export]
macro_rules! impl_units {
    ( $t:ty, { $( $x:ident => $y:expr ),+ } ) => {
        paste! {
            $(
                fn $x(value: f64) -> Box<dyn Value> {
                    Box::new($t {
                        value: value * $y.factor(),
                        unit: $y,
                    })
                }

                fn [<as_ $x>](&self) -> Box<dyn Value> {
                    Box::new($t {
                        value: self.value,
                        unit: $y,
                    })
                }
            )+
        }
    };
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let mm = Length::millimeters(5.);
        println!("1. {} mm => {} inches", mm, mm.as_inches());
        let inches = Length::inches(10.);
        println!("2. {} inches => {} mm", inches, inches.as_millimeters());
        let meters = Length::meters(30.);
        println!("3. {} meters => {} feet", meters, meters.as_feet());
        let feet = Length::feet(25.);
        println!("4. {} feet => {} meters", feet, feet.as_meters());
        let miles = Length::miles(100.);
        println!("5. {} miles => {} kilometers", miles, miles.as_kilometers());
        let kilometers = Length::kilometers(88.);
        println!("6. {} kilometers => {} miles", kilometers, kilometers.as_miles());
        let light_years = Length::light_years(12.);
        println!("7. {} light_years => {} miles", light_years, light_years.as_miles());
        let miles = Length::miles(9.3e7);
        println!("8. {} miles => {} light_years", miles, miles.as_light_years());
        let light_years = Length::light_years(5.);
        println!("9. {} light_years => {} parsecs", light_years, light_years.as_parsecs());
        let parsecs = Length::parsecs(3.);
        println!("10. {} parsecs => {} light_years", parsecs, parsecs.as_light_years());
        let au = Length::au(2.);
        println!("11. {} au => {} miles", au, au.as_miles());
        let miles = Length::miles(10000.);
        println!("12. {} miles => {} au", miles, miles.as_au());
        let degrees = Angle::degrees(180.);
        println!("13. {} degrees => {} radians", degrees, degrees.as_radians());
        let radians = Angle::radians(2.5);
        println!("14. {} radians => {} degrees", radians, radians.as_degrees());
    }
}

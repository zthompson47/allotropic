use std::{
    fmt::{Display, Formatter, Result},
    ops::Mul,
};

fn main() {
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

    /*
    let inches = Length::inches(47.);
    println!("{:?} {}", &inches, &inches);
    let area = inches.clone() * inches;
    println!("{:?}", area);
    */
}

#[derive(Clone, Debug)]
struct Length {
    value: f64,
    unit: Unit,
}

#[derive(Debug)]
struct Angle {
    value: f64,
    unit: Unit,
}

impl Angle {
    l!(
        Angle, {
            degrees => Unit::Degree,
            radians => Unit::Radian
        }
    );
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value / self.unit.factor())
    }
}

impl Mul for Length {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Length {
            value: self.value * rhs.value,
            unit: self.unit * rhs.unit,
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value / self.unit.factor())
    }
}

use paste::paste;
#[macro_export]
macro_rules! l {
    ( $t:ty, { $( $x:ident => $y:expr ),+ } ) => {
        paste! {
            $(
                fn $x(value: f64) -> Self {
                    $t {
                        value: value * $y.factor(),
                        unit: $y,
                    }
                }

                fn [<as_ $x>](&self) -> Self {
                    $t {
                        value: self.value,
                        unit: $y,
                    }
                }
            )+
        }
    };
}

impl Length {
    l!(
        Length, {
            au => Unit::Au,
            feet => Unit::Foot,
            inches => Unit::Inch,
            kilometers => Unit::Kilometer,
            light_years => Unit::LightYear,
            meters => Unit::Meter,
            miles => Unit::Mile,
            millimeters => Unit::Millimeter,
            parsecs => Unit::Parsec
        }
    );
}

#[derive(Clone, Debug)]
enum Unit {
    Au,
    Degree,
    Foot,
    Inch,
    Kilometer,
    LightYear,
    Meter,
    Mile,
    Millimeter,
    Mul(Box<Unit>, Box<Unit>),
    Parsec,
    Radian,
}

impl Unit {
    fn factor(&self) -> f64 {
        match self {
            Unit::Au => 1.495979e11,
            Unit::Degree => 1.745329e-2,
            Unit::Foot => 12. * 0.0254,
            Unit::Inch => 0.0254,
            Unit::Kilometer => 1000.,
            Unit::LightYear => 9.4607304725808e15,
            Unit::Meter => 1.,
            Unit::Mile => 5280. * 12. * 0.0254,
            Unit::Millimeter => 0.001,
            Unit::Mul(_, _) => 1.,
            Unit::Parsec => 3.085678e16,
            Unit::Radian => 1.,
        }
    }
}

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Unit::Mul(Box::new(self), Box::new(rhs))
    }
}

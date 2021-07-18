use ntz::prelude::*;
use ntz::si::*;
use ntz::temperature::*;
use ntz::{angle as a, time as t};

use float_eq::assert_float_eq;

#[test]
fn ch1() {
    // 1.
    let left = millimeters(5.).to_inches();
    let right = inches(0.196850);
    let tol = inches(1e-6);
    assert_float_eq!(left, right, abs <= tol);
    // 2.
    assert_float_eq!(
        inches(10.).to_millimeters(),
        millimeters(254.),
        abs <= miles(0.1)
    );
    // 3.
    assert_float_eq!(meters(30.).to_feet(), feet(98.425197), abs <= feet(1e-6));
    // 4.
    assert_float_eq!(feet(25.).to_meters(), meters(7.62), abs <= meters(0.01));
    // 5.
    assert_float_eq!(
        miles(100.).to_kilometers(),
        kilometers(160.9344),
        abs <= kilometers(1e-4)
    );
    // 6.
    assert_float_eq!(
        kilometers(88.).to_miles(),
        miles(54.680665),
        abs <= miles(1e-6)
    );
    // 7.
    assert_float_eq!(
        light_years(12.).to_miles(),
        miles(7.044e13),
        abs <= miles(1.0e12)
    );
    // 8.
    assert_float_eq!(
        miles(9.3e7).to_light_years(),
        light_years(1.5843e-5),
        abs <= light_years(1.0e-7)
    );
    // 9.
    assert_float_eq!(
        light_years(5.).to_parsecs(),
        parsecs(1.534),
        abs <= parsecs(1e-3)
    );
    // 10.
    assert_float_eq!(
        parsecs(3.).to_light_years(),
        light_years(9.7784),
        abs <= light_years(1.0e-2)
    );
    // 11.
    assert_float_eq!(
        astronomical_units(2.).to_miles(),
        miles(1.8580e8),
        abs <= miles(1.0e6)
    );
    // 12.
    assert_float_eq!(
        miles(10_000.).to_astronomical_units(),
        astronomical_units(1.076426E-4),
        abs <= astronomical_units(1.0e-7)
    );
    // 13.
    #[allow(clippy::approx_constant)]
    let pi = 3.141593;
    assert_float_eq!(
        a::degrees(180.).to_radians(),
        a::radians(pi),
        abs <= a::radians(1e-6)
    );
    // 14.
    assert_float_eq!(
        a::radians(2.5).to_degrees(),
        a::degrees(143.239449),
        abs <= a::degrees(1e-6)
    );
    // 15.
    assert_float_eq!(
        a::hours(2.).to_degrees(),
        a::degrees(30.0),
        abs <= a::degrees(0.1)
    );
    // 16.
    assert_float_eq!(
        a::degrees(156.4).to_hours(),
        a::hours(10.42),
        abs <= a::hours(0.01)
    );
    // 17.
    assert_float_eq!(
        Value::from_hms((10., 25., 11.)).to_hours(),
        t::hours(10.419722),
        abs <= t::hours(1e-6)
    );
    // 18.
    assert_float_eq!(
        t::hours(20.352).to_hms_tuple(),
        (20., 21., 7.2),
        abs <= (1., 1., 0.1)
    );
    // 19.
    assert_float_eq!(
        Value::from_dms((13., 4., 10.)).to_degrees(),
        a::degrees(13.069444),
        abs <= a::degrees(1e-6)
    );
    // 20.
    assert_float_eq!(
        a::degrees(-0.508333).to_dms_tuple(),
        (-0., -30., -30.00),
        abs <= (1., 1., 0.01)
    );
    // 21.
    assert_float_eq!(
        Value::from_dms((300., 20., 0.)).to_degrees(),
        a::degrees(300.333333),
        abs <= a::degrees(1e-6)
    );
    // 22.
    assert_float_eq!(
        a::degrees(10.2958).to_dms_tuple(),
        (10., 17., 44.88),
        abs <= (1., 1., 1e-2)
    );
    // 23.
    assert_float_eq!(
        celsius(100.).to_fahrenheit(),
        fahrenheit(212.00),
        abs <= fahrenheit(0.01)
    );
    // 24.
    assert_float_eq!(
        fahrenheit(32.).to_celsius(),
        celsius(0.00),
        abs <= celsius(0.01)
    );
}

use ntz::angle::*;
use ntz::si::*;

use float_eq::assert_float_eq;

#[test]
fn ch1() {
    // 1.
    let left = millimeters(5.).to_inches();
    let right = inches(0.19685);
    let tol = inches(0.00001);
    assert_float_eq!(left, right, abs <= tol);
    // 2.
    assert_float_eq!(
        inches(10.).to_millimeters(),
        millimeters(254.),
        abs <= miles(0.1)
    );
    // 3.
    assert_float_eq!(
        meters(30.).to_feet(),
        feet(98.425197),
        abs <= feet(0.000001)
    );
    // 4.
    assert_float_eq!(feet(25.).to_meters(), meters(7.62), abs <= meters(0.01));
    // 5.
    assert_float_eq!(
        miles(100.).to_kilometers(),
        kilometers(160.9344),
        abs <= kilometers(0.0001)
    );
    // 6.
    assert_float_eq!(
        kilometers(88.).to_miles(),
        miles(54.680665),
        abs <= miles(0.000001)
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
    //assert_float_eq!(180.0f64.to_radians(), pi, abs <= 1e-6);
    assert_float_eq!(
        degrees(180.).to_radians(),
        radians(pi),
        abs <= radians(1e-6)
    );
    // 14.
    //assert_float_eq!(2.5f64.to_degrees(), 143.239449, abs <= 1e-6);
    assert_float_eq!(
        radians(2.5).to_degrees(),
        degrees(143.239449),
        abs <= degrees(1e-6)
    );
    // 15.
    assert_float_eq!(hours(2.).to_degrees(), degrees(30.), abs <= degrees(1.));
    // 16.
    assert_float_eq!(degrees(156.4).to_hours(), hours(10.42), abs <= hours(0.01));
    // 17.
    //assert_float_eq!(
}

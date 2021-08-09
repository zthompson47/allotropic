//use std::ops::{Add, Div, Mul, Sub},

//use float_eq::derive_float_eq;

/*
#[derive_float_eq(
    ulps_tol = "JulianDayUlps",
    debug_ulps_diff = "JulianDayDebugUlpsDiff",
    all_tol = "f64"
)]
*/

/*
impl Add<f64> for JulianDay {
    type Output = JulianDay;

    fn add(self, rhs: f64) -> Self::Output {
        JulianDay(self.0 + rhs)
    }
}

impl Add<JulianDay> for f64 {
    type Output = f64;

    fn add(self, rhs: JulianDay) -> Self::Output {
        self + rhs.0
    }
}

impl Add<JulianDay> for JulianDay {
    type Output = JulianDay;

    fn add(self, rhs: JulianDay) -> Self::Output {
        JulianDay(self.0 + rhs.0)
    }
}

impl Sub<f64> for JulianDay {
    type Output = JulianDay;

    fn sub(self, rhs: f64) -> Self::Output {
        JulianDay(self.0 - rhs)
    }
}

impl Sub<JulianDay> for f64 {
    type Output = f64;

    fn sub(self, rhs: JulianDay) -> Self::Output {
        self - rhs.0
    }
}

impl Sub<JulianDay> for JulianDay {
    type Output = JulianDay;

    fn sub(self, rhs: JulianDay) -> Self::Output {
        JulianDay(self.0 - rhs.0)
    }
}

impl Mul<f64> for JulianDay {
    type Output = JulianDay;

    fn mul(self, rhs: f64) -> Self::Output {
        JulianDay(self.0 * rhs)
    }
}

impl Mul<JulianDay> for f64 {
    type Output = f64;

    fn mul(self, rhs: JulianDay) -> Self::Output {
        self * rhs.0
    }
}

impl Mul<JulianDay> for JulianDay {
    type Output = JulianDay;

    fn mul(self, rhs: JulianDay) -> Self::Output {
        JulianDay(self.0 * rhs.0)
    }
}

impl Div<f64> for JulianDay {
    type Output = JulianDay;

    fn div(self, rhs: f64) -> Self::Output {
        JulianDay(self.0 / rhs)
    }
}

impl Div<JulianDay> for f64 {
    type Output = f64;

    fn div(self, rhs: JulianDay) -> Self::Output {
        self / rhs.0
    }
}

impl Div<JulianDay> for JulianDay {
    type Output = JulianDay;

    fn div(self, rhs: JulianDay) -> Self::Output {
        JulianDay(self.0 / rhs.0)
    }
}
*/

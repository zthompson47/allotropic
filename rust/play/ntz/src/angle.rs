use crate::prelude::*;

unit! {
    Angle {
        // TODO: need to be dimensionless
        Degree => (degrees, "°", 1f64.to_radians()),
        Hour => (hours, "'", 15f64.to_radians()),
        Minute => (minutes, "\"", 6f64.to_radians()),
        Radian => (radians, "rad", 1.)
    }
}

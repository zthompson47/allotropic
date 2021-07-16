use crate::prelude::*;

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

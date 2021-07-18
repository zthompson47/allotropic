use crate::prelude::*;

unit! {
    Temperature {
        Fahrenheit => (fahrenheit, "°F", 1. / 1.8, -32. / 1.8),
        Celsius => (celsius, "°C", 1.)
    }
}

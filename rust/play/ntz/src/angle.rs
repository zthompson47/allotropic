use crate::prelude::*;

unit! {
    Angle {
        // TODO: need to be dimensionless
        Degree => (degrees, "°", 1_f64.to_radians()),
        Hour => (hours, "h", 15_f64.to_radians()),
        Minute => (minutes, "'", (1_f64 / 60_f64).to_radians()),
        Radian => (radians, "rad", 1.),
        Second => (seconds, "\"", (1_f64 / 3600_f64).to_radians()),
        Dms => (dms, "hms", 1.)
    }
}

impl Value<Angle> {
    pub fn from_dms(dms: (f64, f64, f64)) -> Self {
        Value {
            value: (degrees(dms.0) + minutes(dms.1) + seconds(dms.2))
                .to_dms()
                .value,
            unit: Angle::Dms(1.),
        }
    }
}

impl Value<Angle> {
    pub fn to_dms_tuple(self) -> (f64, f64, f64) {
        let d = self.to_degrees().value; // TODO: maybe add `% 360.`
        let d_trunc = d.trunc();
        let d_rem = d.fract();
        let m = degrees(d_rem).to_minutes().value;
        let m_trunc = m.trunc();
        let m_rem = m.fract();
        let s = minutes(m_rem).to_seconds().value;
        (d_trunc, minutes(m_trunc).value, seconds(s).value)
    }
}

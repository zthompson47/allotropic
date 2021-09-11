use crate::prelude::*;

unit! {
    Time {
        Hour => (hours, "h", 60. * 60.),
        Minute => (minutes, "min", 60.),
        Second => (seconds, "s", 1.)
        //Hms => (hms, "hms", 1.)
    }
}

impl Value<Time> {
    pub fn from_hms(hms: (f64, f64, f64)) -> Self {
        Value {
            value: (hours(hms.0) + minutes(hms.1) + seconds(hms.2))
                .to_seconds()
                .value,
            unit: Time::Second(1.),
        }
    }

    pub fn to_hms(self) -> (f64, f64, f64) {
        let h = self.to_hours().value;
        let h_trunc = h.trunc();
        let h_rem = h.fract();
        let m = hours(h_rem).to_minutes().value;
        let m_trunc = m.trunc();
        let m_rem = m.fract();
        let s = minutes(m_rem).to_seconds().value;
        (
            h_trunc,
            minutes(m_trunc).value,
            seconds(s).value,
        )
    }
}

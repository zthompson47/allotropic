use chrono::{Date, DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};

pub trait JulianDay {
    fn to_julian_day(&self) -> f64;
    fn from_julian_day(day: f64) -> Self;
    fn day_of_week(&self) -> Day;
    fn days_into_year(&self) -> u32;
    fn from_days_into_year(year: i32, days: u32) -> Self;
    fn is_leap_year(&self) -> bool;
}

pub trait ToSidereal: JulianDay + Timelike {
    fn to_gst(&self) -> Self;
    fn to_lst(&self, longitude: f64) -> Self;
}

#[derive(Clone)]
pub struct SiderealDateTime<Tz: TimeZone> {
    datetime: DateTime<Tz>,
}

impl ToSidereal for DateTime<Utc> {
    #[allow(clippy::many_single_char_names)]
    fn to_gst(&self) -> DateTime<Utc> {
        let jd = self.date().to_julian_day();
        let jd0 = self
            .date()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .to_julian_day();
        let days = jd - jd0;
        let t = (jd0 - 2_415_020.0) / 36_525.;
        let r = 6.6460656 + 2400.051262 * t + 0.00002581 * t.powi(2);
        let b = 24. - r + 24. * (self.year() as f64 - 1900.);
        let t0 = 0.0657098 * days - b;
        let ut = self.decimal_hour();
        let mut gst = t0 + 1.002738 * ut;
        //let mut day_offset = 0i32;
        if gst < 0. {
            gst += 24.;
            //day_offset = -1;
        }
        if gst >= 24. {
            gst -= 24.;
            //day_offset = 1;
        };
        let (h, m, s) = decimal_hour_to_hms(gst);

        //NaiveDate::from_ymd(self.year(), self.month(), self.day()).and_hms(h, m, s)
        self.with_hour(h)
            .unwrap()
            .with_minute(m)
            .unwrap()
            .with_second(s)
            .unwrap()
    }

    fn to_lst(&self, longitude: f64) -> Self {
        let gst = self.to_gst().decimal_hour();
        let adjustment = longitude / 15.;
        let mut lst = gst + adjustment;
        if lst < 0. {
            lst += 24.;
        }
        if lst >= 24. {
            lst -= 24.;
        }
        let (h, m, s) = decimal_hour_to_hms(lst);

        self.with_hour(h)
            .unwrap()
            .with_minute(m)
            .unwrap()
            .with_second(s)
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<u8> for Day {
    fn from(d: u8) -> Self {
        match d {
            0 => Day::Sunday,
            1 => Day::Monday,
            2 => Day::Tuesday,
            3 => Day::Wednesday,
            4 => Day::Thursday,
            5 => Day::Friday,
            6 => Day::Saturday,
            _ => panic!("invalid day number: {}", d),
        }
    }
}

pub struct HourAngle(pub f64, pub f64, pub f64);

pub fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

impl JulianDay for Date<Utc> {
    #[allow(clippy::many_single_char_names)]
    fn to_julian_day(&self) -> f64 {
        self.and_hms(0, 0, 0).to_julian_day()
    }

    fn from_julian_day(jd: f64) -> Self {
        DateTime::from_julian_day(jd).date()
    }

    fn day_of_week(&self) -> Day {
        let jd = self.to_julian_day();
        let a = (jd + 1.5) / 7.;
        let b = 7. * a.fract();
        Day::from(b.round() as u8)
    }

    fn days_into_year(&self) -> u32 {
        let t = if self.is_leap_year() { 1. } else { 2. };
        ((275. * self.month() as f64 / 9.).trunc() - t * ((self.month() as f64 + 9.) / 12.).trunc()
            + self.day() as f64
            - 30.) as u32
    }

    fn is_leap_year(&self) -> bool {
        is_leap_year(self.year())
    }

    fn from_days_into_year(year: i32, days: u32) -> Self {
        let a = if is_leap_year(year) { 1523. } else { 1889. };
        let b = ((days as f64 + a - 122.1) / 365.25).trunc();
        let c = days as f64 + a - (365.25 * b).trunc();
        let e = (c / 30.6001).trunc();
        let month = if e < 13.5 { e - 1. } else { e - 13. };
        let day = c - (30.6001 * e).trunc();

        Utc.ymd(year as i32, month as u32, day as u32)
    }
}

impl JulianDay for DateTime<Utc> {
    #[allow(clippy::many_single_char_names)]
    fn to_julian_day(&self) -> f64 {
        let (mut y, mut m, mut d) = (self.year(), self.month(), self.day() as f64);
        if m <= 2 {
            y -= 1;
            m += 12;
        }
        let t = if y < 0 { 0.74 } else { 0. };
        let gregorian = self >= &Utc.ymd(1582, 10, 15).and_hms(0, 0, 0);
        let b = if gregorian {
            let a = (y as f64 / 100.).trunc();
            2. - a + (a / 4.).trunc()
        } else {
            0.
        };

        // Add fractional day
        d += self.decimal_day();
        //d += self.hour() as f64 / 24.
        //    + self.minute() as f64 / (24. * 60.)
        //    + self.second() as f64 / (24. * 60. * 60.);

        b + (365.25 * y as f64 - t as f64).trunc()
            + (30.6001 * (m as f64 + 1.)).trunc()
            + d as f64
            + 1_720_994.5
    }

    #[allow(clippy::many_single_char_names)]
    fn from_julian_day(jd: f64) -> Self {
        let jd1 = jd + 0.5;
        let i = jd1.trunc();
        let f = jd1.fract();
        let b = if i > 2_299_160. {
            let a = ((i - 1_867_216.25) / 36_524.25).trunc();
            i + 1. + a - (a / 4.).trunc()
        } else {
            1.
        };
        let c = b + 1524.;
        let d = ((c - 122.1) / 365.25).trunc();
        let e = (365.25 * d).trunc();
        let g = ((c - e) / 30.6001).trunc();
        let day = c - e + f - (30.6001 * g).trunc();
        let month = if g < 13.5 {
            g - 1.
        } else if g > 13.5 {
            g - 13.
        } else {
            panic!()
        };
        let year = if month > 2.5 {
            d - 4716.
        } else if month < 2.5 {
            d - 4715.
        } else {
            panic!()
        };

        // Convert fractional day to hms
        let fd = jd.fract();
        let mut fd_h = fd * 24. + 12.;
        if fd_h >= 24. {
            fd_h -= 24.;
        }
        let fd_m = fd_h.fract() * 60.;
        let fd_s = fd_m.fract() * 60.;
        let (h, m, s) = (fd_h.trunc(), fd_m.trunc(), fd_s.trunc());

        Utc.ymd(year as i32, month as u32, day as u32)
            .and_hms(h as u32, m as u32, s as u32)
    }

    fn day_of_week(&self) -> Day {
        self.date().day_of_week()
    }

    fn days_into_year(&self) -> u32 {
        self.date().days_into_year()
    }

    fn is_leap_year(&self) -> bool {
        self.date().is_leap_year()
    }

    fn from_days_into_year(year: i32, days: u32) -> Self {
        Date::from_days_into_year(year, days).and_hms(0, 0, 0)
    }
}

fn decimal_hour_to_hms(hour: f64) -> (u32, u32, u32) {
    let minute = hour.fract() * 60.;
    let second = minute.fract() * 60.;

    (
        hour.trunc() as u32,
        minute.trunc() as u32,
        second.round() as u32,
    )
}

/*
fn to_hms(time: f64) -> (u32, u32, u32) {
    let mut fd_h = time * 24. + 12.;
    if fd_h >= 24. {
        fd_h -= 24.;
    }
    let fd_m = fd_h.fract() * 60.;
    let fd_s = fd_m.fract() * 60.;

    (
        fd_h.trunc() as u32,
        fd_m.trunc() as u32,
        fd_s.trunc() as u32,
    )
}
*/

trait DecimalTime: Timelike {
    fn decimal_day(&self) -> f64;
    fn decimal_hour(&self) -> f64;
}

impl<Tz: TimeZone> DecimalTime for DateTime<Tz> {
    fn decimal_day(&self) -> f64 {
        self.hour() as f64 / 24.
            + self.minute() as f64 / (24. * 60.)
            + self.second() as f64 / (24. * 60. * 60.)
            + self.nanosecond() as f64 / (24. * 60. * 60. * 1e9)
    }

    fn decimal_hour(&self) -> f64 {
        self.hour() as f64
            + self.minute() as f64 / 60.
            + self.second() as f64 / (60. * 60.)
            + self.nanosecond() as f64 / (60. * 60. * 1e9)
    }
}

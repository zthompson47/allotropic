use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use chrono::{Date, DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

#[derive(Clone, Copy, Debug)]
pub struct JulianDay(pub f64);

impl Deref for JulianDay {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for JulianDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Tz: TimeZone> From<Date<Tz>> for JulianDay {
    fn from(d: Date<Tz>) -> Self {
        JulianDay::from(d.and_hms(0, 0, 0))
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for JulianDay {
    #[allow(clippy::many_single_char_names)]
    fn from(dt: DateTime<Tz>) -> Self {
        let utc = dt.with_timezone(&Utc);
        let (mut y, mut m, mut d) = (utc.year(), utc.month(), utc.day() as f64);
        if m <= 2 {
            y -= 1;
            m += 12;
        }
        let t = if y < 0 { 0.74 } else { 0. };
        let gregorian = utc >= Utc.ymd(1582, 10, 15).and_hms(0, 0, 0);
        let b = if gregorian {
            let a = (y as f64 / 100.).trunc();
            2. - a + (a / 4.).trunc()
        } else {
            0.
        };

        // Add fractional day
        d += utc.decimal_day();

        JulianDay(
            b + (365.25 * y as f64 - t as f64).trunc()
                + (30.6001 * (m as f64 + 1.)).trunc()
                + d as f64
                + 1_720_994.5,
        )
    }
}

impl From<JulianDay> for DateTime<Utc> {
    #[allow(clippy::many_single_char_names)]
    fn from(jd: JulianDay) -> Self {
        let jd1 = *jd + 0.5;
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
}

pub trait DateUtil {
    fn day_of_week(&self) -> Day;
    fn days_into_year(&self) -> u32;
    fn from_days_into_year(year: i32, days: u32) -> Self;
    fn is_leap_year(&self) -> bool;
}

pub trait TimeUtil {
    fn with_dst(&self) -> Self;
}

impl<Tz: TimeZone> TimeUtil for DateTime<Tz> {
    fn with_dst(&self) -> Self {
        self.clone() + Duration::hours(1)
    }
}

/*
pub trait ToSidereal: DateUtil + Timelike {
    //fn to_gst(&self) -> Self;
    fn to_lst(&self, longitude: f64) -> Self;
}
*/

#[derive(Debug, PartialEq)]
pub struct SiderealDateTime<Tz: TimeZone>(pub DateTime<Tz>);

/*
impl Deref for GreenwichSiderealTime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
*/

impl<Tz: TimeZone> Deref for SiderealDateTime<Tz> {
    type Target = DateTime<Tz>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
impl<Tz: TimeZone> From<DateTime<Tz>> for GreenwichSiderealTime {
    #[allow(clippy::many_single_char_names)]
    fn from(dt: DateTime<Tz>) -> Self {
        let jd = JulianDay::from(dt.date());
        let jd0 = JulianDay::from(dt.date().with_month(1).unwrap().with_day(1).unwrap());
        let days = *jd - *jd0;
        let t = (*jd0 - 2_415_020.0) / 36_525.;
        let r = 6.6460656 + 2400.051262 * t + 0.00002581 * t.powi(2);
        let b = 24. - r + 24. * (dt.year() as f64 - 1900.);
        let t0 = 0.0657098 * days - b;
        let ut = dt.decimal_hour();
        let mut gst = t0 + 1.002738 * ut;
        // TODO: Is this right?
        let mut day_offset: i32 = 0;
        if gst < 0. {
            gst += 24.;
            day_offset = -1;
            println!("-------->>>> gst < 0");
        }
        if gst >= 24. {
            gst -= 24.;
            day_offset = 1;
            println!("-------->>>> gst >= 24");
        };
        let (h, m, s) = decimal_hour_to_hms(gst);

        GreenwichSiderealTime(
            dt.with_day((dt.day() as i32 + day_offset) as u32)
                .unwrap()
                .with_hour(h)
                .unwrap()
                .with_minute(m)
                .unwrap()
                .with_second(s)
                .unwrap()
                .naive_local(),
        )
    }
}
*/

impl<Tz: TimeZone> From<DateTime<Tz>> for SiderealDateTime<Tz> {
    #[allow(clippy::many_single_char_names)]
    fn from(dt: DateTime<Tz>) -> Self {
        let jd = JulianDay::from(dt.date());
        let jd0 = JulianDay::from(dt.date().with_month(1).unwrap().with_day(1).unwrap());
        let days = *jd - *jd0;
        let t = (*jd0 - 2_415_020.0) / 36_525.;
        let r = 6.6460656 + 2400.051262 * t + 0.00002581 * t.powi(2);
        let b = 24. - r + 24. * (dt.year() as f64 - 1900.);
        let t0 = 0.0657098 * days - b;
        let ut = dt.decimal_hour();
        let gst = t0 + 1.002738 * ut;

        let result = SiderealDateTime(dt.date().and_hms(0, 0, 0) + decimal_hour_to_duration(gst));
        println!("decimal_hour: {}", decimal_hour_to_duration(gst) / 60 / 60);
        println!("dt.hour: {}", dt.hour());
        println!("result.hour: {}", result.hour());
        result

        /*
        let mut day_offset: i32 = 0;
        if gst < 0. {
            gst += 24.;
            day_offset = -1;
            println!("-------->>>> gst < 0");
        }
        if gst >= 24. {
            gst -= 24.;
            day_offset = 1;
            println!("-------->>>> gst >= 24");
        };
        let (h, m, s) = decimal_hour_to_hms(gst);
        println!("{:?}", (h, m, s));

        SiderealDateTime(
            dt.with_day((dt.day() as i32 + day_offset) as u32)
                .unwrap()
                .with_hour(h)
                .unwrap()
                .with_minute(m)
                .unwrap()
                .with_second(s)
                .unwrap(),
        )
        */
    }
}

impl<Tz: TimeZone + Debug> SiderealDateTime<Tz> {
    pub fn to_lst(&self, longitude: f64) -> Self {
        //let gst = self.to_gst().decimal_hour();
        let gst = self.with_timezone(&Utc).decimal_hour();
        let adjustment = longitude / 15.;
        let lst = gst + adjustment;

        SiderealDateTime(self.date().and_hms(0, 0, 0) + decimal_hour_to_duration(lst))

        /*
        let mut day_offset = 0;
        if lst < 0. {
            lst += 24.;
            day_offset = -1;
        }
        if lst >= 24. {
            lst -= 24.;
            day_offset = 1;
        }
        let (h, m, s) = decimal_hour_to_hms(lst);

        SiderealDateTime(
            self.with_day((self.day() as i32 + day_offset) as u32)
                .unwrap()
                .with_hour(h)
                .unwrap()
                .with_minute(m)
                .unwrap()
                .with_second(s)
                .unwrap(),
        )
        */
    }

    pub fn to_gst(&self, longitude: f64) -> SiderealDateTime<Utc> {
        let lst = self.decimal_hour();
        let adjust = longitude / 15.;
        let gst = lst - adjust;

        SiderealDateTime(
            self.with_timezone(&Utc).date().and_hms(0, 0, 0) + decimal_hour_to_duration(gst),
        )
    }
}

impl SiderealDateTime<Utc> {
    pub fn to_ut(&self) -> DateTime<Utc> {
        let jd = JulianDay::from(self.date());
        let jd0 = JulianDay::from(self.date().with_month(1).unwrap().with_day(1).unwrap());
        let days = *jd - *jd0;
        let t = (*jd0 - 2_415_020.0) / 36_525.;
        let r = 6.6460656 + 2400.051262 * t + 0.00002581 * t.powi(2);
        let b = 24. - r + 24. * (self.year() as f64 - 1900.);
        let mut t0 = 0.0657098 * days - b;
        if t0 < 0. {
            t0 += 24.;
        }
        if t0 >= 24. {
            t0 -= 24.
        }
        let gst = self.decimal_hour();
        let mut a = gst - t0;
        if a < 0. {
            a += 24.;
        }
        let ut = 0.997270 * a;

        self.0.date().and_hms(0, 0, 0) + decimal_hour_to_duration(ut)
    }
}

/*
impl ToSidereal for DateTime<Utc> {
    #[allow(clippy::many_single_char_names)]
    fn to_gst(&self) -> DateTime<Utc> {
        let jd = JulianDay::from(self.date());
        let jd0 = JulianDay::from(self.date().with_month(1).unwrap().with_day(1).unwrap());
        let days = *jd - *jd0;
        let t = (*jd0 - 2_415_020.0) / 36_525.;
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
*/

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

impl DateUtil for Date<Utc> {
    fn day_of_week(&self) -> Day {
        //let jd = self.to_julian_day();
        let jd = JulianDay::from(*self);
        let a = (*jd + 1.5) / 7.;
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

impl DateUtil for DateTime<Utc> {
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

/*
fn decimal_hour_to_hms(mut hour: f64) -> (u32, u32, u32) {
    let mut minute = hour.fract() * 60.;
    let mut second = (minute.fract() * 60.).round();

    if second >= 60. {
        second = 0.;
        minute += 1.;
    }
    if minute >= 60. {
        minute = 0.;
        hour += 1.;
    }
    if hour >= 24. {
        hour = 0.;
    }

    (hour.trunc() as u32, minute.trunc() as u32, second as u32)
}
*/

fn decimal_hour_to_duration(hour: f64) -> Duration {
    let minute = hour.fract() * 60.;
    let second = minute.fract() * 60.;
    let nanosecond = second.fract() * 1e9;

    Duration::hours(hour.trunc() as i64)
        + Duration::minutes(minute.trunc() as i64)
        + Duration::seconds(second.trunc() as i64)
        + Duration::nanoseconds(nanosecond.round() as i64)
}

pub trait DecimalTime: Timelike {
    fn decimal_day(&self) -> f64;
    fn decimal_hour(&self) -> f64;
    fn with_rounded_seconds(&self) -> Self;
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

    fn with_rounded_seconds(&self) -> Self {
        let seconds = self.nanosecond() as f64 * 1e-9;
        let duration = Duration::seconds(seconds.round() as i64);
        self.with_nanosecond(0).unwrap() + duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use float_eq::assert_float_eq;

    #[test]
    fn deref_sidereal_time() {
        let datetime = Utc.ymd(2021, 5, 26).and_hms(12, 1, 0);
        let st = SiderealDateTime(datetime);
        assert_eq!(st.date(), datetime.date());
        assert_float_eq!(
            *JulianDay::from(*st),
            *JulianDay::from(datetime),
            abs <= 1e-3
        );
    }
}

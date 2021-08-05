use chrono::{TimeZone, Utc};

use celestial::{Day, JulianDay, ToSidereal};

fn main() {
    println!("------------------------");
    let d = Utc.ymd(2011, 9, 11);
    let jd = d.to_julian_day();
    println!("jd: {}", jd);
    let a = (jd + 1.5) / 7.;
    println!("a: {}", a);
    let b = 7. * a.fract();
    println!("b: {}", b);
    let day = Day::from(b.round() as u8);
    println!("day is: {:?}", day);

    println!("------------------------");
    let d2 = Utc.ymd(2010, 1, 1);
    println!("{:?} is julian day {}", d2, d2.to_julian_day());

    println!("------------------------");
    let utc = Utc.ymd(2010, 2, 7).and_hms(23, 30, 0);
    println!("==>> result: {:?}", utc.to_gst());
}

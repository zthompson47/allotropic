use chrono::{TimeZone, Utc};

use celestial::{Day, JulianDay};

fn main() {
    let d = Utc.ymd(2011, 9, 11);
    let jd = d.to_julian_day();
    println!("jd: {}", jd);
    let a = (jd + 1.5) / 7.;
    println!("a: {}", a);
    let b = 7. * a.fract();
    println!("b: {}", b);
    let day = Day::from(b.round() as u8);
    println!("day is: {:?}", day);

    let d2 = Utc.ymd(2010, 1, 1);
    println!("{:?} is julian day {}", d2, d2.to_julian_day());
}

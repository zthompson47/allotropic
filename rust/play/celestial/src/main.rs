use chrono::{Datelike, Duration, TimeZone, Timelike, Utc};

use celestial::{Day, JulianDay, SiderealDateTime};

fn main() {
    if let Some(func) = std::env::args().nth(1) {
        match func.as_str() {
            "gst_vs_ut" => gst_vs_ut(),
            "jd" => jd(),
            "sd" => sd(),
            "date_math" => date_math(),
            "lst_to_gst" => lst_to_gst(),
            _ => {}
        }
    }
}

fn date_math() {
    let date = Utc.ymd(2021, 8, 31).and_hms(23, 59, 59);
    let dur = Duration::seconds(1);
    println!("{}", date + dur);
}

fn gst_vs_ut() {
    let date = Utc.ymd(2021, 8, 7);
    for hour in 0..24 {
        for minute in 0..60 {
            let time = date.and_hms(hour, minute, 0);
            print!(
                "{} {}:{}:{}",
                time.day(),
                time.hour(),
                time.minute(),
                time.second()
            );
            let st = SiderealDateTime::from(time);
            println!(
                " -- {} {}:{}:{}",
                st.day(),
                st.hour(),
                st.minute(),
                st.second()
            );
        }
    }
}

fn jd() {
    println!("------------------------");
    let jd = JulianDay::from(Utc.ymd(2011, 9, 11));
    println!("jd: {}", jd);
    let a = (*jd + 1.5) / 7.;
    println!("a: {}", a);
    let b = 7. * a.fract();
    println!("b: {}", b);
    let day = Day::from(b.round() as u8);
    println!("day is: {:?}", day);

    println!("------------------------");
    let d2 = Utc.ymd(2010, 1, 1);
    println!("{:?} is julian day {}", d2, JulianDay::from(d2));
}

fn sd() {
    println!("------------------------");
    let utc = Utc.ymd(2010, 2, 7).and_hms(23, 30, 0);
    println!("==>> result: {:?}", SiderealDateTime::from(utc));
}

fn lst_to_gst() {
    let date = SiderealDateTime(Utc.ymd(2000, 7, 5).and_hms(23, 23, 41));
    assert_eq!(Utc.ymd(2000, 7, 5).and_hms(20, 3, 41), *date.to_gst(50.));
}

use chrono::{Date, DateTime, Datelike, TimeZone, Timelike, Utc};
use float_eq::assert_float_eq;

use celestial::{is_leap_year, Day, JulianDay};

#[test]
fn ch3_q1_q4() {
    assert!(is_leap_year(1984));
    assert!(!is_leap_year(1974));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
}

#[test]
fn ch3_q5() {
    let t = Utc.ymd(2010, 11, 1);
    assert_float_eq!(t.to_julian_day(), 2_455_501.5, abs <= 0.1);
}

#[test]
fn ch3_q6() {
    let t = Utc.ymd(2015, 5, 10).and_hms(6, 0, 0);
    assert_float_eq!(t.to_julian_day(), 2_457_152.75, abs <= 0.1);
}

#[test]
fn ch3_q7() {
    let t = Utc.ymd(2015, 5, 10).and_hms(18, 0, 0);
    assert_float_eq!(t.to_julian_day(), 2_457_153.25, abs <= 0.1);
}

#[test]
fn ch3_q8() {
    let j = DateTime::from_julian_day(2_369_915.5);
    assert_eq!(
        (
            j.year(),
            j.month(),
            j.day(),
            j.hour(),
            j.minute(),
            j.second()
        ),
        (1776, 7, 4, 0, 0, 0)
    );
}

#[test]
fn ch3_q9() {
    let j = DateTime::from_julian_day(2_455_323.0);
    assert_eq!(
        (
            j.year(),
            j.month(),
            j.day(),
            j.hour(),
            j.minute(),
            j.second()
        ),
        (2010, 5, 6, 12, 0, 0)
    );
}

#[test]
fn ch3_q10() {
    let j = DateTime::from_julian_day(2_456_019.37);
    assert_eq!(
        (
            j.year(),
            j.month(),
            j.day(),
            j.hour(),
            j.minute(),
            j.second()
        ),
        (2012, 4, 1, 20, 52, 48)
    );
}

#[test]
fn ch3_q11() {
    assert_eq!(Utc.ymd(1776, 7, 4).day_of_week(), Day::Thursday);
}

#[test]
fn ch3_q12() {
    assert_eq!(Utc.ymd(2011, 9, 11).day_of_week(), Day::Sunday);
    assert_eq!(Utc.ymd(2011, 9, 12).day_of_week(), Day::Monday);
}

#[test]
fn ch3_q13() {
    assert_eq!(Utc.ymd(2009, 10, 30).days_into_year(), 303);
}

#[test]
fn ch3_q14() {
    assert_eq!(Date::from_days_into_year(1900, 250), Utc.ymd(1900, 9, 7));
}

#[test]
fn pre_gregorian_dates() {
    // TODO: Figure out how to handle the julian->gregorian transition?  Or maybe it's fine?
    let d = Utc.ymd(1200, 1, 11).and_hms(0, 47, 22);
    assert_float_eq!(d.to_julian_day(), 2159367.5328935, abs <= 1e-7);
    // Gregorian switchover
    assert_float_eq!(Utc.ymd(1582, 10, 4).to_julian_day(), 2299159.5, abs <= 0.1);
    //assert_float_eq!(Utc.ymd(1582, 10, 5).to_julian_day(), 2299150.5, abs <= 0.1);
    assert_float_eq!(Utc.ymd(1582, 10, 15).to_julian_day(), 2299160.5, abs <= 0.1);
    assert_float_eq!(Utc.ymd(1582, 10, 16).to_julian_day(), 2299161.5, abs <= 0.1);
}

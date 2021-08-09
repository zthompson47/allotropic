use chrono::{Date, DateTime, TimeZone, Utc};
use chrono_tz::{
    Etc::GMTMinus4,
    US::Eastern,
};
use float_eq::assert_float_eq;

use celestial::{is_leap_year, DateUtil, Day, DecimalTime, JulianDay, SiderealDateTime, TimeUtil};

#[test]
fn ch3_q1_q4() {
    assert!(is_leap_year(1984));
    assert!(!is_leap_year(1974));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
}

#[test]
fn ch3_q5() {
    let jd = JulianDay::from(Utc.ymd(2010, 11, 1));
    assert_float_eq!(*jd, 2_455_501.5, abs <= 0.1);
}

#[test]
fn ch3_q6() {
    let jd = JulianDay::from(Utc.ymd(2015, 5, 10).and_hms(6, 0, 0));
    assert_float_eq!(*jd, 2_457_152.75, abs <= 0.1);
}

#[test]
fn ch3_q7() {
    let jd = JulianDay::from(Utc.ymd(2015, 5, 10).and_hms(18, 0, 0));
    assert_float_eq!(*jd, 2_457_153.25, abs <= 0.1);
}

#[test]
fn ch3_q8() {
    let dt = DateTime::from(JulianDay(2_369_915.5));
    assert_eq!(dt, Utc.ymd(1776, 7, 4).and_hms(0, 0, 0));
}

#[test]
fn ch3_q9() {
    let dt = DateTime::from(JulianDay(2_455_323.0));
    assert_eq!(dt, Utc.ymd(2010, 5, 6).and_hms(12, 0, 0));
}

#[test]
fn ch3_q10() {
    let dt = DateTime::from(JulianDay(2_456_019.37));
    assert_eq!(dt, Utc.ymd(2012, 4, 1).and_hms(20, 52, 48));
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
fn ch3_q15() {
    let date = Eastern.ymd(2014, 12, 12).and_hms(20, 0, 0);
    assert_eq!(
        date.with_timezone(&Utc),
        Utc.ymd(2014, 12, 13).and_hms(1, 0, 0)
    );
    assert_eq!(
        SiderealDateTime::from(date)
            .with_timezone(&Utc)
            .with_rounded_seconds(),
        Utc.ymd(2014, 12, 13).and_hms(6, 26, 34)
    );
    assert_eq!(
        SiderealDateTime::from(date)
            .to_lst(-77.)
            //.with_timezone(&Utc)
            .with_rounded_seconds(),
        Eastern.ymd(2014, 12, 13).and_hms(1, 18, 34)
    );
}

#[test]
fn ch3_q16() {
    let date = SiderealDateTime(GMTMinus4.ymd(2000, 7, 5).and_hms(5, 54, 20));

    assert_eq!(Utc.ymd(2000, 7, 5).and_hms(1, 54, 20), *date.to_gst(60.));
    assert_eq!(
        date.to_gst(60.).to_ut().with_rounded_seconds(),
        Utc.ymd(2000, 7, 5).and_hms(7, 0, 0)
    );

    let e = Eastern.ymd(2021, 8, 8).and_hms(12, 0, 0);
    println!("e: {:?} {:?}", e, e.with_timezone(&Utc));

    let gm4 = GMTMinus4.ymd(2000, 7, 5).and_hms(12, 0, 0);
    println!("gm4: {:?} {:?}", gm4, gm4.with_timezone(&Utc));

    assert_eq!(
        date.to_gst(60.)
            .to_ut()
            .with_timezone(&GMTMinus4)
            .with_dst()
            .with_rounded_seconds(),
        GMTMinus4.ymd(2000, 7, 5).and_hms(12, 0, 0)
    );
}

/*
#[test]
fn gst_with_day_offset() {
    let date = Utc.ymd(2014, 12, 12).and_hms(23, 59, 59);
    assert_eq!(
        date.to_gst(),
        NaiveDate::from_ymd(2014, 12, 13).and_hms(5, 26, 23)
    );
    // TODO: find gst conversions that offset days in both directions.. confirm above
    // day flipover
}
*/

#[test]
fn pre_gregorian_dates() {
    // TODO: Figure out how to handle the julian->gregorian transition?  Or maybe it's fine?
    let dt = Utc.ymd(1200, 1, 11).and_hms(0, 47, 22);
    assert_float_eq!(*JulianDay::from(dt), 2159367.5328935, abs <= 1e-7);
    // Gregorian switchover
    assert_float_eq!(
        *JulianDay::from(Utc.ymd(1582, 10, 4)),
        2299159.5,
        abs <= 0.1
    );
    //assert_float_eq!(
    //    *JulianDay::from(Utc.ymd(1582, 10, 5)),
    //    2299150.5,
    //    abs <= 0.1
    //);
    assert_float_eq!(
        *JulianDay::from(Utc.ymd(1582, 10, 15)),
        2299160.5,
        abs <= 0.1
    );
    assert_float_eq!(
        *JulianDay::from(Utc.ymd(1582, 10, 16)),
        2299161.5,
        abs <= 0.1
    );
}

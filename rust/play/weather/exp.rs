#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::collections::HashMap;
use chrono::{Date, DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use clap::App;
use colorgrad::{Color, CustomGradient};
use crossterm::style::{self, Stylize};
use structopt::StructOpt;
use sunrise::sunrise_sunset;
use wthr::{
    client::ApiClient,
    config::{Base, Config},
    error::Result,
    APP, NWS_API,
};
const USR: &str = "zach@allotropic.com";
const LAT: f64 = 42.440932990492946;
const LON: f64 = -76.52462385924595;
struct Opt {
    /// Hourly forecast
    #[structopt(short)]
    hourly: bool,
    /// Latitude
    #[structopt(env, long = "lat")]
    latitude: f64,
    /// Longitude
    #[structopt(env, long = "lon")]
    longitude: f64,
    /// Api key
    #[structopt(env, short = "k", conflicts_with = "profile")]
    api_key: Option<String>,
    /// Profile
    profile: Option<String>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Opt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Opt {
                hourly: ref __self_0_0,
                latitude: ref __self_0_1,
                longitude: ref __self_0_2,
                api_key: ref __self_0_3,
                profile: ref __self_0_4,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Opt");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "hourly", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "latitude",
                    &&(*__self_0_1),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "longitude",
                    &&(*__self_0_2),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "api_key",
                    &&(*__self_0_3),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "profile",
                    &&(*__self_0_4),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Opt {
    #[inline]
    fn default() -> Opt {
        Opt {
            hourly: ::core::default::Default::default(),
            latitude: ::core::default::Default::default(),
            longitude: ::core::default::Default::default(),
            api_key: ::core::default::Default::default(),
            profile: ::core::default::Default::default(),
        }
    }
}
#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)]
impl ::structopt::StructOpt for Opt {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("wthr");
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        Opt {
            hourly: matches.is_present("hourly"),
            latitude: matches
                .value_of("latitude")
                .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                .unwrap(),
            longitude: matches
                .value_of("longitude")
                .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                .unwrap(),
            api_key: matches
                .value_of("api-key")
                .map(|s| ::std::str::FromStr::from_str(s).unwrap()),
            profile: matches
                .value_of("profile")
                .map(|s| ::std::str::FromStr::from_str(s).unwrap()),
        }
    }
}
#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)]
impl ::structopt::StructOptInternal for Opt {
    fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
        {
            let app = app;
            let app = app.arg(
                ::structopt::clap::Arg::with_name("hourly")
                    .takes_value(false)
                    .multiple(false)
                    .help("Hourly forecast")
                    .short("hourly"),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("latitude")
                    .takes_value(true)
                    .multiple(false)
                    .required(true)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: f64| ())
                            .map_err(|e| e.to_string())
                    })
                    .help("Latitude")
                    .env("LATITUDE")
                    .long("lat"),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("longitude")
                    .takes_value(true)
                    .multiple(false)
                    .required(true)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: f64| ())
                            .map_err(|e| e.to_string())
                    })
                    .help("Longitude")
                    .env("LONGITUDE")
                    .long("lon"),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("api-key")
                    .takes_value(true)
                    .multiple(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: String| ())
                            .map_err(|e| e.to_string())
                    })
                    .help("Api key")
                    .env("API_KEY")
                    .short("k")
                    .conflicts_with("profile"),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("profile")
                    .takes_value(true)
                    .multiple(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: String| ())
                            .map_err(|e| e.to_string())
                    })
                    .help("Profile"),
            );
            app.version("0.1.0")
        }
    }
    fn is_subcommand() -> bool {
        false
    }
}
fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let opt = Opt::from_args();
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", " ", " ", "\n"],
                    &match (&opt.latitude, &opt.longitude, &opt.api_key) {
                        (arg0, arg1, arg2) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                        ],
                    },
                ));
            };
            let mut client = ApiClient::builder()
                .base_url(NWS_API)
                .api_key(APP, USR)
                .build()?;
            let point = client.get_point(<[_]>::into_vec(box [LAT, LON])).await?;
            let forecast = client
                .get_forecast_from_url(if opt.hourly {
                    point.properties.forecast_hourly
                } else {
                    point.properties.forecast
                })
                .await?;
            let max_wind_column_len = forecast
                .properties
                .periods
                .iter()
                .map(|x| x.wind_speed.len())
                .max()
                .unwrap_or(0);
            let max_time_column_len = forecast
                .properties
                .periods
                .iter()
                .map(|x| {
                    DateTime::<Utc>::from_utc(x.start_time.naive_utc(), Utc)
                        .format("%A %l%P")
                        .to_string()
                        .len()
                })
                .max()
                .unwrap_or(0);
            let temp_grad = CustomGradient::new()
                .colors(&[
                    Color::from_rgb_u8(255, 255, 255),
                    Color::from_rgb_u8(66, 66, 255),
                    Color::from_rgb_u8(66, 255, 66),
                    Color::from_rgb_u8(255, 130, 66),
                ])
                .domain(&[0., 32., 72., 84.])
                .build()?;
            let mut sun_times: HashMap<Date<Utc>, (DateTime<Utc>, DateTime<Utc>)> = HashMap::new();
            for period in forecast.properties.periods {
                let time = DateTime::from_utc(period.start_time.naive_utc(), Utc);
                let date = time.date();
                let (sunrise, sunset) = sun_times.entry(date).or_insert({
                    let (rise, set) =
                        sunrise_sunset(LAT, LON, date.year(), date.month(), date.day());
                    (Utc.timestamp(rise, 0), Utc.timestamp(set, 0))
                });
                let time_color = if (time > *sunrise && time < *sunset)
                    || time.hour() == sunrise.hour()
                    || time.hour() == sunset.hour()
                {
                    style::Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 0,
                    }
                } else {
                    style::Color::Rgb {
                        r: 100,
                        g: 0,
                        b: 255,
                    }
                };
                let local_time = Local.from_utc_datetime(&time.naive_utc());
                let time = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
                        &[""],
                        &match (
                            &local_time.format("%A %l%P").to_string(),
                            &max_time_column_len,
                        ) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::from_usize(arg1),
                            ],
                        },
                        &[::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Right,
                                flags: 0u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Param(1usize),
                            },
                        }],
                    ));
                    res
                };
                let (r, g, b, _a) = temp_grad.at(period.temperature as f64).rgba_u8();
                let temp_color = style::Color::Rgb { r, g, b };
                let temp = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["", "\u{b0}"],
                        &match (&period.temperature, &period.temperature_unit) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ));
                    res
                };
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                        &["", " ", " ", " ", " ", "\n"],
                        &match (
                            &time.with(time_color),
                            &temp.with(temp_color),
                            &period.wind_speed,
                            &max_wind_column_len,
                            &period.wind_direction,
                            &period.short_forecast,
                        ) {
                            (arg0, arg1, arg2, arg3, arg4, arg5) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg4, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg5, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::from_usize(arg3),
                            ],
                        },
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 2usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Right,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(5usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 3usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Left,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Is(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 4usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                    ));
                };
            }
            Ok(())
        })
}
